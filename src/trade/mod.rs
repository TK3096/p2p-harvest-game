use std::sync::Arc;

use anyhow::Result;
use async_channel::Sender;
use iroh::{
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler, Router},
    Endpoint, EndpointId,
};
use n0_future::{boxed::BoxStream, Stream};
use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, Mutex};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

use crate::{
    crop::Crop,
    event::trade::{AcceptTradeEvent, TradeEvent, TradeItemType},
    game::GameState,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeItem {
    pub item_type: TradeItemType,
    pub amount: Option<u32>,
    pub crop: Option<Crop>,
}

#[derive(Clone, Debug)]
pub struct Trade {
    game_state: Arc<Mutex<GameState>>,
    event_sender: broadcast::Sender<AcceptTradeEvent>,
}

impl Trade {
    pub const ALPN: &[u8] = b"/p2p-harvest-game/trade/1.0.0";

    pub fn new(
        event_sender: broadcast::Sender<AcceptTradeEvent>,
        game_state: Arc<Mutex<GameState>>,
    ) -> Self {
        Self {
            event_sender,
            game_state,
        }
    }

    async fn handle_connection(
        self,
        connection: Connection,
    ) -> std::result::Result<(), AcceptError> {
        let endpoint_id = connection.remote_id();
        self.event_sender
            .send(AcceptTradeEvent::Connected { endpoint_id })
            .ok();

        let res = self.handle_connection_0(&connection).await;
        let error = res.as_ref().err().map(|err| err.to_string());

        self.event_sender
            .send(AcceptTradeEvent::Closed { endpoint_id, error })
            .ok();

        res
    }

    async fn handle_connection_0(
        &self,
        connection: &Connection,
    ) -> std::result::Result<(), AcceptError> {
        let endpoint_id = connection.remote_id();

        let (mut send, mut recv) = connection.accept_bi().await?;

        // Read the trade item from sender
        println!("[RECEIVER] Waiting to receive trade item...");
        let mut buffer = Vec::new();
        tokio::io::copy(&mut recv, &mut buffer).await?;
        println!("[RECEIVER] Received {} bytes", buffer.len());

        let trade_item: TradeItem =
            serde_json::from_slice(&buffer).map_err(|err| AcceptError::from_err(err))?;
        println!("[RECEIVER] Parsed trade item: {:?}", trade_item.item_type);

        self.event_sender
            .send(AcceptTradeEvent::TradeReceived {
                endpoint_id,
                item_type: trade_item.item_type.clone(),
                amount: trade_item.amount,
                crop: trade_item.crop.clone(),
            })
            .ok();

        // Prepare acceptance response
        let acceptance = serde_json::json!({
            "status": "trade_accepted",
            "item_type": trade_item.item_type,
            "amount": trade_item.amount,
            "crop": trade_item.crop,
        });

        let acceptance_bytes =
            serde_json::to_vec(&acceptance).map_err(|err| AcceptError::from_err(err))?;

        // Send response to sender
        println!(
            "[RECEIVER] Sending acceptance response ({} bytes)...",
            acceptance_bytes.len()
        );
        tokio::io::copy(&mut acceptance_bytes.as_slice(), &mut send).await?;
        send.finish()?;
        println!("[RECEIVER] Response sent and stream finished");

        // Update the shared game state
        {
            let mut gs = self.game_state.lock().await;
            match trade_item.item_type {
                TradeItemType::Money => {
                    if let Some(amount) = trade_item.amount {
                        gs.player.money += amount;
                    }
                }
                TradeItemType::Crop => {
                    if let Some(crop) = trade_item.crop {
                        gs.player.inventory.push(crop);
                    }
                }
            }

            // Save the updated state to disk
            println!("[RECEIVER] Saving updated game state...");
            gs.save().unwrap();
            println!("[RECEIVER] Game state saved successfully");
        }

        self.event_sender
            .send(AcceptTradeEvent::TradeCompleted { endpoint_id })
            .ok();

        // Wait a bit to ensure sender receives the response before connection closes
        println!("[RECEIVER] Waiting before closing connection...");
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        println!("[RECEIVER] Done processing trade");

        Ok(())
    }
}

impl ProtocolHandler for Trade {
    async fn accept(&self, connection: Connection) -> std::result::Result<(), AcceptError> {
        self.clone().handle_connection(connection).await
    }
}

#[derive(Clone)]
pub struct TradeNode {
    router: Router,
    accept_events: broadcast::Sender<AcceptTradeEvent>,
    game_state: Arc<Mutex<GameState>>,
}

impl TradeNode {
    pub async fn spawn(game_state: GameState) -> Result<Self> {
        let endpoint_builder = iroh::Endpoint::builder()
            .alpns(vec![Trade::ALPN.to_vec()])
            .bind()
            .await?;

        let game_state_arc = Arc::new(Mutex::new(game_state));
        let (event_sender, _) = broadcast::channel(128);
        let trade = Trade::new(event_sender.clone(), game_state_arc.clone());
        let router = Router::builder(endpoint_builder)
            .accept(Trade::ALPN, trade)
            .spawn();

        Ok(Self {
            router,
            accept_events: event_sender,
            game_state: game_state_arc,
        })
    }

    pub fn get_endpoint(&self) -> &Endpoint {
        self.router.endpoint()
    }

    pub fn get_game_state(&self) -> Arc<Mutex<GameState>> {
        self.game_state.clone()
    }

    async fn initiate_trade(
        endpoint: &Endpoint,
        endpoint_id: EndpointId,
        event_sender: Sender<TradeEvent>,
        trade_item: TradeItem,
        game_state: Arc<Mutex<GameState>>,
    ) -> Result<()> {
        // Connect to the receiver
        println!("[SENDER] Connecting to receiver...");
        let connection = endpoint.connect(endpoint_id, Trade::ALPN).await?;
        println!("[SENDER] Connected successfully");
        event_sender.send(TradeEvent::Connected).await?;

        // Open a bidirectional stream
        println!("[SENDER] Opening bidirectional stream...");
        let (mut send_stream, mut recv_stream) = connection.open_bi().await?;
        println!("[SENDER] Stream opened");

        // Serialize and send the trade item
        let payload = serde_json::to_vec(&trade_item)?;
        event_sender
            .send(TradeEvent::TradeProposed {
                item_type: trade_item.item_type.clone(),
                amount: trade_item.amount,
                crop: trade_item.crop.clone(),
            })
            .await?;

        println!("[SENDER] Sending trade item ({} bytes)...", payload.len());
        tokio::io::copy(&mut payload.as_slice(), &mut send_stream).await?;
        send_stream.finish()?;
        println!("[SENDER] Trade item sent, stream finished");

        // Wait for response from receiver
        println!("[SENDER] Waiting for response...");
        let mut buffer = Vec::new();
        let bytes_read = tokio::io::copy(&mut recv_stream, &mut buffer).await?;
        println!("[SENDER] Received {} bytes", bytes_read);

        if bytes_read == 0 || buffer.is_empty() {
            return Err(anyhow::anyhow!(
                "No response from receiver - connection closed early"
            ));
        }

        // Parse the response
        println!("[SENDER] Parsing response...");
        let res: serde_json::Value = serde_json::from_slice(&buffer)?;
        println!("[SENDER] Response parsed: {:?}", res);

        if res["status"] == "trade_accepted" {
            println!("[SENDER] Trade accepted! Updating game state...");
            // Update sender's game state
            let mut gs = game_state.lock().await;

            match trade_item.item_type {
                TradeItemType::Money => {
                    if let Some(amount) = trade_item.amount {
                        if gs.player.money >= amount {
                            gs.player.money -= amount;
                        } else {
                            return Err(anyhow::anyhow!("Insufficient funds"));
                        }
                    }
                }
                TradeItemType::Crop => {
                    if let Some(crop) = &trade_item.crop {
                        if let Some(pos) = gs.player.inventory.iter().position(|c| c.id == crop.id)
                        {
                            gs.player.inventory.remove(pos);
                        } else {
                            return Err(anyhow::anyhow!("Crop not found in inventory"));
                        }
                    }
                }
            }

            // Save the updated state
            println!("[SENDER] Saving updated game state...");
            gs.save()?;
            println!("[SENDER] Game state saved");

            event_sender
                .send(TradeEvent::TradeAccepted {
                    item_type: trade_item.item_type,
                    amount: trade_item.amount,
                    crop: trade_item.crop,
                })
                .await?;
        } else {
            return Err(anyhow::anyhow!("Trade was not accepted by receiver"));
        }

        // Close the connection gracefully
        println!("[SENDER] Closing connection...");
        connection.close(0u8.into(), b"trade complete");
        println!("[SENDER] Trade completed successfully");

        Ok(())
    }

    pub fn trade(
        &self,
        endpoint_id: EndpointId,
        trade_item: TradeItem,
    ) -> impl Stream<Item = TradeEvent> + Unpin + use<> {
        let (event_sender, event_receiver) = async_channel::bounded(16);
        let endpoint = self.router.endpoint().clone();
        let game_state = self.game_state.clone();

        tokio::spawn(async move {
            println!("[SENDER] Task spawned, starting trade...");
            let res = Self::initiate_trade(
                &endpoint,
                endpoint_id,
                event_sender.clone(),
                trade_item,
                game_state,
            )
            .await;
            let error = res.as_ref().err().map(|err| err.to_string());

            event_sender.send(TradeEvent::Closed { error }).await.ok();
        });

        Box::pin(event_receiver)
    }

    pub fn accept(&self) -> BoxStream<AcceptTradeEvent> {
        let receiver = self.accept_events.subscribe();
        Box::pin(BroadcastStream::new(receiver).filter_map(|event| event.ok()))
    }
}

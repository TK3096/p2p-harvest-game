use std::sync::Arc;

use anyhow::Result;
use async_channel::Sender;
use iroh::{
    Endpoint, EndpointId,
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler, Router},
};
use n0_future::{Stream, boxed::BoxStream, task};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, broadcast};
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

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

#[derive(Debug, Clone)]
pub struct Trade {
    game_state: GameState,
    event_sender: broadcast::Sender<AcceptTradeEvent>,
}

impl Trade {
    pub const ALPN: &[u8] = b"/p2p-harvest-game/trade/1.0.0";

    pub fn new(event_sender: broadcast::Sender<AcceptTradeEvent>, game_state: GameState) -> Self {
        Self {
            event_sender,
            game_state,
        }
    }

    async fn handle_connection(
        mut self,
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
        &mut self,
        connection: &Connection,
    ) -> std::result::Result<(), AcceptError> {
        let endpoint_id = connection.remote_id();

        let (mut send, mut recv) = connection.accept_bi().await?;

        let mut buffer = Vec::new();
        tokio::io::copy(&mut recv, &mut buffer).await?;

        let trade_item: TradeItem =
            serde_json::from_slice(&buffer).map_err(|err| AcceptError::from_err(err))?;

        self.event_sender
            .send(AcceptTradeEvent::TradeReceived {
                endpoint_id,
                item_type: trade_item.item_type.clone(),
                amount: trade_item.amount,
                crop: trade_item.crop.clone(),
            })
            .ok();

        let acceptance = serde_json::json!({
            "status": "trade_accpeted",
            "item_type": trade_item.item_type,
            "amount": trade_item.amount,
            "crop": trade_item.crop,
        });

        let acceptance_bytes =
            serde_json::to_vec(&acceptance).map_err(|err| AcceptError::from_err(err))?;
        tokio::io::copy(&mut acceptance_bytes.as_slice(), &mut send).await?;
        send.finish()?;

        match trade_item.item_type {
            TradeItemType::Money => {
                if let Some(amount) = trade_item.amount {
                    self.game_state.player.money += amount;
                }
            }
            TradeItemType::Crop => {
                if let Some(crop) = trade_item.crop {
                    self.game_state.player.inventory.push(crop);
                }
            }
        }

        self.event_sender
            .send(AcceptTradeEvent::TradeCompleted { endpoint_id })
            .ok();

        connection.close(1u8.into(), b"done");

        Ok(())
    }
}

impl ProtocolHandler for Trade {
    async fn accept(&self, connection: Connection) -> std::result::Result<(), AcceptError> {
        self.clone().handle_connection(connection).await
    }
}

#[derive(Debug, Clone)]
pub struct TradeNode {
    router: Router,
    accept_events: broadcast::Sender<AcceptTradeEvent>,
}

impl TradeNode {
    pub async fn spawn(game_state: GameState) -> Result<Self> {
        let endpoint_id = iroh::Endpoint::builder()
            .alpns(vec![Trade::ALPN.to_vec()])
            .bind()
            .await?;
        let (event_sender, _) = broadcast::channel(128);
        let trade = Trade::new(event_sender.clone(), game_state.clone());
        let router = Router::builder(endpoint_id)
            .accept(Trade::ALPN, trade)
            .spawn();

        Ok(Self {
            router,
            accept_events: event_sender,
        })
    }

    pub fn get_endpoint(&self) -> &Endpoint {
        self.router.endpoint()
    }

    async fn initiate_trade(
        endpoint: &Endpoint,
        endpoint_id: EndpointId,
        event_sender: Sender<TradeEvent>,
        trade_item: TradeItem,
        game_state: Arc<Mutex<GameState>>,
    ) -> Result<()> {
        let connection = endpoint.connect(endpoint_id, Trade::ALPN).await?;
        event_sender.send(TradeEvent::Connected).await?;

        let (mut send_stream, mut recv_stream) = connection.accept_bi().await?;
        let payload = serde_json::to_vec(&trade_item)?;

        event_sender
            .send(TradeEvent::TradeProposed {
                item_type: trade_item.item_type.clone(),
                amount: trade_item.amount,
                crop: trade_item.crop.clone(),
            })
            .await?;
        tokio::io::copy(&mut payload.as_slice(), &mut send_stream).await?;
        send_stream.finish()?;

        let mut buffer = Vec::new();
        tokio::io::copy(&mut recv_stream, &mut buffer).await?;

        let res: serde_json::Value = serde_json::from_slice(&buffer)?;

        if res["state"] == "trade_accepted" {
            let mut gs = game_state.lock().await;

            match trade_item.item_type {
                TradeItemType::Money => {
                    if let Some(amount) = trade_item.amount {
                        gs.player.money -= amount;
                    }
                }
                TradeItemType::Crop => {
                    if let Some(crop) = &trade_item.crop {
                        if let Some(pos) = gs.player.inventory.iter().position(|c| c.id == crop.id)
                        {
                            gs.player.inventory.remove(pos);
                        }
                    }
                }
            }

            event_sender
                .send(TradeEvent::TradeAccepted {
                    item_type: trade_item.item_type,
                    amount: trade_item.amount,
                    crop: trade_item.crop,
                })
                .await?;
        }

        connection.close(1u8.into(), b"done");

        Ok(())
    }

    pub fn trade(
        &self,
        endpoint_id: EndpointId,
        trade_item: TradeItem,
        game_state: Arc<Mutex<GameState>>,
    ) -> impl Stream<Item = TradeEvent> + Unpin + use<> {
        let (event_sender, event_receiver) = async_channel::bounded(16);
        let endpoint = self.router.endpoint().clone();

        task::spawn(async move {
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

use anyhow::Result;
use async_channel::Sender;
use iroh::{
    Endpoint, EndpointId,
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler, Router},
};
use n0_future::{Stream, boxed::BoxStream, task};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeItem {
    pub item_name: String,
    pub quantity: u32,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum AcceptEvent {
    Connected {
        endpoint_id: EndpointId,
    },
    TradeReceived {
        endpoint_id: EndpointId,
        item_name: String,
        quantity: u32,
        message: Option<String>,
    },
    TradeCompleted {
        endpoint_id: EndpointId,
    },
    Closed {
        endpoint_id: EndpointId,
        error: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum TradeEvent {
    Connected,
    TradeRequest { item_name: String, quantity: u32 },
    TradeAccepted { item_name: String, quantity: u32 },
    Closed { error: Option<String> },
}

#[derive(Debug, Clone)]
pub struct Trade {
    event_sender: broadcast::Sender<AcceptEvent>,
}

impl Trade {
    pub const ALPN: &[u8] = b"/p2p-harvest-game/trade/1.0.0";

    pub fn new(event_sender: broadcast::Sender<AcceptEvent>) -> Self {
        Self { event_sender }
    }

    async fn handle_connection(
        self,
        connection: Connection,
    ) -> std::result::Result<(), AcceptError> {
        let endpoint_id = connection.remote_id();
        self.event_sender
            .send(AcceptEvent::Connected { endpoint_id })
            .ok();

        let res = self.handle_connection_0(&connection).await;
        let error = res.as_ref().err().map(|err| err.to_string());

        self.event_sender
            .send(AcceptEvent::Closed { endpoint_id, error })
            .ok();

        res
    }

    async fn handle_connection_0(
        &self,
        connection: &Connection,
    ) -> std::result::Result<(), AcceptError> {
        let endpoint_id = connection.remote_id();

        let (mut send, mut recv) = connection.accept_bi().await?;

        let mut buffer = Vec::new();
        tokio::io::copy(&mut recv, &mut buffer).await?;

        let trade_item: TradeItem =
            serde_json::from_slice(&buffer).map_err(|err| AcceptError::from_err(err))?;

        self.event_sender
            .send(AcceptEvent::TradeReceived {
                endpoint_id,
                item_name: trade_item.item_name.clone(),
                quantity: trade_item.quantity,
                message: trade_item.message.clone(),
            })
            .ok();

        let acceptance = serde_json::json!({
            "status": "trade_accpeted",
            "item_name": trade_item.item_name,
            "quantity": trade_item.quantity
        });

        let acceptance_bytes =
            serde_json::to_vec(&acceptance).map_err(|err| AcceptError::from_err(err))?;
        tokio::io::copy(&mut acceptance_bytes.as_slice(), &mut send).await?;
        send.finish()?;

        self.event_sender
            .send(AcceptEvent::TradeCompleted { endpoint_id })
            .ok();

        connection.closed().await;

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
    accept_events: broadcast::Sender<AcceptEvent>,
}

impl TradeNode {
    pub async fn spawn() -> Result<Self> {
        let endpoint_id = iroh::Endpoint::builder()
            .alpns(vec![Trade::ALPN.to_vec()])
            .bind()
            .await?;
        let (event_sender, _event_receiver) = broadcast::channel(128);
        let trade = Trade::new(event_sender.clone());
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

    pub fn trade(
        &self,
        endpoint_id: EndpointId,
        value: TradeItem,
    ) -> impl Stream<Item = TradeEvent> + Unpin + use<> {
        let (event_sender, event_receiver) = async_channel::bounded(16);
        let endpoint = self.router.endpoint().clone();

        task::spawn(async move {
            let res =
                Self::initiate_trade(&endpoint, endpoint_id, value, event_sender.clone()).await;
            let error = res.as_ref().err().map(|err| err.to_string());

            event_sender.send(TradeEvent::Closed { error }).await.ok();
        });

        Box::pin(event_receiver)
    }

    pub fn accept(&self) -> BoxStream<AcceptEvent> {
        let receiver = self.accept_events.subscribe();
        Box::pin(BroadcastStream::new(receiver).filter_map(|event| event.ok()))
    }

    async fn initiate_trade(
        endpoint: &Endpoint,
        endpoint_id: EndpointId,
        value: TradeItem,
        event_sender: Sender<TradeEvent>,
    ) -> Result<()> {
        let connection = endpoint.connect(endpoint_id, Trade::ALPN).await?;
        event_sender.send(TradeEvent::Connected).await?;

        let (mut send_stream, mut recv_stream) = connection.open_bi().await?;

        let data = serde_json::to_vec(&value)?;

        event_sender
            .send(TradeEvent::TradeRequest {
                item_name: value.item_name.clone(),
                quantity: value.quantity,
            })
            .await?;
        tokio::io::copy(&mut data.as_slice(), &mut send_stream).await?;
        send_stream.finish()?;

        let mut buffer = Vec::new();
        tokio::io::copy(&mut recv_stream, &mut buffer).await?;

        let res: serde_json::Value = serde_json::from_slice(&buffer)?;

        if res["status"] == "trade_accpeted" {
            event_sender
                .send(TradeEvent::TradeAccepted {
                    item_name: value.item_name,
                    quantity: value.quantity,
                })
                .await?;
        }

        connection.close(1u8.into(), b"done");

        Ok(())
    }
}

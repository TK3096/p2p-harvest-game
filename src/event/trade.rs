use iroh::EndpointId;
use serde::{Deserialize, Serialize};

use crate::crop::Crop;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeItemType {
    Crop,
    Money,
}

#[derive(Debug, Clone)]
pub enum TradeEvent {
    Connected,
    TradeProposed {
        item_type: TradeItemType,
        amount: Option<u32>,
        crop: Option<Crop>,
    },
    TradeAccepted {
        item_type: TradeItemType,
        amount: Option<u32>,
        crop: Option<Crop>,
    },
    Closed {
        error: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub enum AcceptTradeEvent {
    Connected {
        endpoint_id: EndpointId,
    },
    TradeReceived {
        endpoint_id: EndpointId,
        item_type: TradeItemType,
        amount: Option<u32>,
        crop: Option<Crop>,
    },
    TradeCompleted {
        endpoint_id: EndpointId,
    },
    Closed {
        endpoint_id: EndpointId,
        error: Option<String>,
    },
}

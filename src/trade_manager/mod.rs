use std::sync::Arc;

use anyhow::{Context, Result};
use iroh::EndpointId;
use tokio::{runtime::Runtime, sync::Mutex};
use tokio_stream::StreamExt;

use crate::{
    event::trade::{AcceptTradeEvent, TradeEvent, TradeItemType},
    game::GameState,
    trade::{TradeItem, TradeNode},
};

pub struct TradeManager {
    runtime: Runtime,
    trade_node: Option<TradeNode>,
}

impl TradeManager {
    pub fn new() -> Result<Self> {
        let runtime = Runtime::new().context("Failed to create Tokio runtime")?;
        Ok(Self {
            runtime,
            trade_node: None,
        })
    }

    pub fn initialize(&mut self, game_state: GameState) -> Result<()> {
        let trade_node = self
            .runtime
            .block_on(async { TradeNode::spawn(game_state).await })?;

        let endpoint_id = trade_node.get_endpoint().id();
        println!("🔗 Trade Node initialized!");
        println!("📋 Your Endpoint ID: {}", endpoint_id);
        println!("Share this ID with other players to trade!\n");

        self.trade_node = Some(trade_node);
        Ok(())
    }

    pub fn get_endpoint_id(&self) -> Option<EndpointId> {
        self.trade_node
            .as_ref()
            .map(|node| node.get_endpoint().id())
    }

    pub fn send_trade(
        &self,
        remote_endpoint_id: EndpointId,
        trade_item: TradeItem,
        game_state: Arc<Mutex<GameState>>,
    ) -> Result<()> {
        let trade_node = self
            .trade_node
            .as_ref()
            .context("Trade node not initialized")?;

        self.runtime.block_on(async {
            let mut stream = trade_node.trade(remote_endpoint_id, trade_item, game_state);

            while let Some(event) = stream.next().await {
                match event {
                    TradeEvent::Connected => {
                        println!("✅ Connected to peer");
                    }
                    TradeEvent::TradeProposed {
                        item_type,
                        amount,
                        crop,
                    } => match item_type {
                        TradeItemType::Money => {
                            println!("💰 Sending {} coins...", amount.unwrap_or(0));
                        }
                        TradeItemType::Crop => {
                            if let Some(crop) = crop {
                                println!("🌾 Sending {} crop...", crop.name);
                            }
                        }
                    },
                    TradeEvent::TradeAccepted {
                        item_type,
                        amount,
                        crop,
                    } => match item_type {
                        TradeItemType::Money => {
                            println!("✅ Trade accepted! Sent {} coins", amount.unwrap_or(0));
                        }
                        TradeItemType::Crop => {
                            if let Some(crop) = crop {
                                println!("✅ Trade accepted! Sent {}", crop.name);
                            }
                        }
                    },
                    TradeEvent::Closed { error } => {
                        if let Some(err) = error {
                            println!("❌ Trade failed: {}", err);
                        } else {
                            println!("✅ Trade completed successfully!");
                        }
                        break;
                    }
                }
            }
            Ok(())
        })
    }

    pub fn listen_for_trades(&self, timeout_secs: u64) -> Result<()> {
        let trade_node = self
            .trade_node
            .as_ref()
            .context("Trade node not initialized")?;

        println!(
            "👂 Listening for incoming trades (timeout: {} seconds)...\n",
            timeout_secs
        );

        self.runtime.block_on(async {
            let mut stream = trade_node.accept();
            let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(timeout_secs));
            tokio::pin!(timeout);

            loop {
                tokio::select! {
                    Some(event) = stream.next() => {
                        match event {
                            AcceptTradeEvent::Connected { endpoint_id } => {
                                println!("🔗 Peer connected: {}", endpoint_id);
                            }
                            AcceptTradeEvent::TradeReceived {
                                endpoint_id,
                                item_type,
                                amount,
                                crop,
                            } => {
                                println!("📦 Trade received from {}", endpoint_id);
                                match item_type {
                                    TradeItemType::Money => {
                                        println!("💰 Received {} coins!", amount.unwrap_or(0));
                                    }
                                    TradeItemType::Crop => {
                                        if let Some(crop) = crop {
                                            println!("🌾 Received {} crop!", crop.name);
                                        }
                                    }
                                }
                            }
                            AcceptTradeEvent::TradeCompleted { endpoint_id } => {
                                println!("✅ Trade with {} completed!", endpoint_id);
                            }
                            AcceptTradeEvent::Closed { endpoint_id, error } => {
                                if let Some(err) = error {
                                    println!("❌ Connection with {} closed: {}", endpoint_id, err);
                                } else {
                                    println!("👋 Connection with {} closed", endpoint_id);
                                }
                            }
                        }
                    }
                    _ = &mut timeout => {
                        println!("⏰ Listening timeout reached");
                        break;
                    }
                }
            }
            Ok(())
        })
    }
}

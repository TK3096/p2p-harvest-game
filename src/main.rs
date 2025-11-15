mod crop;
mod game;
mod message;
mod player;
mod trade;

use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use iroh::EndpointId;
use tokio_stream::StreamExt;

use crate::{
    game::GameState,
    message::WELCOME_MESSAGE,
    trade::{TradeItem, TradeNode},
};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Start a new game session
    Play,
    /// Trade seed with other players
    Trade {
        endpoint_id: EndpointId,
        item_name: String,
        quantity: u32,
        #[clap(short, long)]
        message: Option<String>,
    },
    /// Open trade requests from other players
    Accept,
    /// Reset the game progress
    Reset,
}

#[tokio::main]
async fn main() -> Result<ExitCode> {
    let args = Args::parse();
    let mut game_state = GameState::load_or_create()?;
    let node = TradeNode::spawn().await?;

    match args.command {
        SubCommands::Play => {
            println!("{}", WELCOME_MESSAGE);
            game_state.play().context("Failed to start game")?;

            println!("\n👋 Thanks for playing!");
        }
        SubCommands::Trade {
            endpoint_id,
            item_name,
            quantity,
            message,
        } => {
            println!("Starting trade...");
            println!("Trading: {} x{}", item_name, quantity);
            println!("To: {}", endpoint_id);
            println!("Message: {}", message.as_deref().unwrap_or("None"));
            println!();

            let trade_item = TradeItem {
                item_name,
                quantity,
                message,
            };

            let mut events = node.trade(endpoint_id, trade_item);
            while let Some(event) = events.next().await {
                match &event {
                    trade::TradeEvent::Connected => {
                        println!("Connected to {}", endpoint_id);
                    }
                    trade::TradeEvent::TradeRequest {
                        item_name,
                        quantity,
                    } => {
                        println!("Trade request: {} x{}", item_name, quantity);
                    }
                    trade::TradeEvent::TradeAccepted {
                        item_name,
                        quantity,
                    } => {
                        println!("Trade accepted: {} x{}", item_name, quantity);
                    }
                    trade::TradeEvent::Closed { error } => {
                        if let Some(err) = error {
                            println!("Trade closed with error: {}", err);
                        } else {
                            println!("Trade completed successfully.");
                        }
                    }
                }
            }
        }
        SubCommands::Accept => {
            println!("Accept trade requests...");
            println!("Youd Node ID: {}", node.get_endpoint().id());
            println!("Waiting for incoming trades...");
            println!();

            let mut events = node.accept();
            while let Some(event) = events.next().await {
                match &event {
                    trade::AcceptEvent::Connected { endpoint_id } => {
                        println!("Connected to {}", endpoint_id);
                    }
                    trade::AcceptEvent::TradeReceived {
                        endpoint_id,
                        item_name,
                        quantity,
                        message,
                    } => {
                        println!("Trade received from: {}", endpoint_id);
                        println!("Trade: {} x{}", item_name, quantity);

                        if let Some(msg) = message {
                            println!("Message: {}", msg);
                        }
                    }
                    trade::AcceptEvent::TradeCompleted { endpoint_id } => {
                        println!("Trade completed with {}", endpoint_id);
                    }
                    trade::AcceptEvent::Closed { endpoint_id, error } => {
                        if let Some(err) = error {
                            println!("Trade with {} closed with error: {}", endpoint_id, err);
                        } else {
                            println!("Trade with {} completed successfully.", endpoint_id);
                        }
                    }
                }
            }
        }
        SubCommands::Reset => {
            game_state.reset()?;
            println!("Game has been reset.");
        }
    }

    Ok(ExitCode::SUCCESS)
}

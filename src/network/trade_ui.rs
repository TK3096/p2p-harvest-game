use anyhow::{Context, Result};
use iroh::EndpointId;
use std::io::{self, StdoutLock, Write};
use std::str::FromStr;

use crate::core::GameEngine;

use super::{
    manager::TradeManager,
    trade_protocol::{TradeItem, TradeItemType},
};

pub fn handle_trade(
    stdout: &mut StdoutLock,
    trade_manager: &TradeManager,
    game_engine: &mut GameEngine,
) -> Result<()> {
    write!(stdout, "🎁 P2P Trade System\r\n")?;
    writeln!(stdout)?;

    write!(stdout, "Select mode:\r\n")?;
    write!(
        stdout,
        "1. Send trade (transfer crops/money to another player)\r\n"
    )?;
    write!(stdout, "2. Receive trade (listen for incoming trades)\r\n")?;
    write!(stdout, "3. Cancel\r\n")?;

    let mut selected = String::new();
    io::stdin().read_line(&mut selected)?;

    match selected.trim().parse::<usize>() {
        Ok(1) => {
            handle_send_trade(stdout, trade_manager, game_engine)?;
        }
        Ok(2) => {
            handle_receive_trade(stdout, trade_manager, game_engine)?;
        }
        Ok(3) => {
            write!(stdout, "❌ Trade cancelled.\r\n")?;
        }
        _ => {
            write!(stdout, "😖 Invalid selection.\r\n")?;
        }
    }

    Ok(())
}

fn handle_send_trade(
    stdout: &mut StdoutLock,
    trade_manager: &TradeManager,
    game_engine: &mut GameEngine,
) -> Result<()> {
    write!(stdout, "\n📤 Send Trade\r\n")?;

    // Get peer endpoint ID
    write!(stdout, "Enter peer's Endpoint ID:\r\n")?;
    let mut endpoint_input = String::new();
    io::stdin().read_line(&mut endpoint_input)?;

    let endpoint_id =
        EndpointId::from_str(endpoint_input.trim()).context("Invalid Endpoint ID format")?;

    // Choose what to send
    write!(stdout, "\nWhat do you want to send?\r\n")?;
    write!(stdout, "1. Money\r\n")?;
    write!(stdout, "2. Crop\r\n")?;

    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;

    let info = game_engine.get_info();

    let trade_item = match choice.trim().parse::<usize>() {
        Ok(1) => {
            // Send money
            write!(stdout, "Enter amount of money to send:\r\n")?;
            let mut amount_input = String::new();
            io::stdin().read_line(&mut amount_input)?;
            let amount = amount_input
                .trim()
                .parse::<u32>()
                .context("Invalid amount")?;

            if info.player_money < amount {
                write!(
                    stdout,
                    "❌ Not enough money! You have {} coins.\r\n",
                    info.player_money
                )?;
                return Ok(());
            }

            TradeItem {
                item_type: TradeItemType::Money,
                amount: Some(amount),
                crop: None,
            }
        }
        Ok(2) => {
            // Send crop
            if info.inventory.is_empty() {
                write!(stdout, "❌ No crops in inventory!\r\n")?;
                return Ok(());
            }

            write!(stdout, "Your inventory:\r\n")?;
            for (index, crop) in info.inventory.iter().enumerate() {
                write!(stdout, "{}. {}\r\n", index + 1, crop.name)?;
            }

            write!(stdout, "Select crop by number:\r\n")?;
            let mut crop_input = String::new();
            io::stdin().read_line(&mut crop_input)?;
            let crop_index = crop_input
                .trim()
                .parse::<usize>()
                .context("Invalid crop selection")?;

            if crop_index == 0 || crop_index > info.inventory.len() {
                write!(stdout, "❌ Invalid selection!\r\n")?;
                return Ok(());
            }

            let crop = info.inventory[crop_index - 1].clone();

            TradeItem {
                item_type: TradeItemType::Crop,
                amount: None,
                crop: Some(crop),
            }
        }
        _ => {
            write!(stdout, "❌ Invalid choice!\r\n")?;
            return Ok(());
        }
    };

    // Perform the trade
    write!(stdout, "\n📡 Initiating trade...\r\n")?;

    trade_manager.send_trade(endpoint_id, trade_item)?;

    // Reload game engine from the shared state in TradeNode
    if let Some(game_engine_arc) = trade_manager.get_game_engine() {
        let rt = tokio::runtime::Runtime::new()?;
        *game_engine = rt.block_on(async { game_engine_arc.lock().await.clone() });
    }

    Ok(())
}

fn handle_receive_trade(
    stdout: &mut StdoutLock,
    trade_manager: &TradeManager,
    game_engine: &mut GameEngine,
) -> Result<()> {
    write!(stdout, "\n📥 Receive Trade\r\n")?;
    write!(stdout, "Waiting for incoming trades...\r\n")?;
    write!(stdout, "(Will timeout after 60 seconds)\r\n\n")?;

    trade_manager.listen_for_trades(60)?;

    // Reload game engine as it may have been updated
    if let Some(game_engine_arc) = trade_manager.get_game_engine() {
        let rt = tokio::runtime::Runtime::new()?;
        *game_engine = rt.block_on(async { game_engine_arc.lock().await.clone() });
    }

    Ok(())
}

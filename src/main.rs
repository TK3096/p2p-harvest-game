use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use p2p_harvest_game::game::GameState;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start a game
    Start,
    /// Reset the game state
    Reset,
}

fn main() -> Result<ExitCode> {
    let args = Args::parse();
    let mut game_state = GameState::load_or_create()?;

    match args.command {
        Command::Start => {
            println!("🌱 Welcome to the P2P Harvest Game! 🌱");
            game_state.start().context("Failed to strat game")?;
        }
        Command::Reset => {
            game_state.reset()?;
            println!("Game state has been reset.");
        }
    }

    Ok(ExitCode::SUCCESS)
}

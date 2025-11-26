use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[cfg(feature = "cli")]
use p2p_harvest_game::cli::{CliApp, GamePersistence};

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

    match args.command {
        Command::Start => {
            println!("🌱 Welcome to the P2P Harvest Game! 🌱");
            let mut app = CliApp::load_or_create()?;
            app.run().context("Failed to run game")?;
        }
        Command::Reset => {
            GamePersistence::reset()?;
            println!("Game state has been reset.");
        }
    }

    Ok(ExitCode::SUCCESS)
}

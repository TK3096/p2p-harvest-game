mod crop;
mod game;
mod message;
mod player;

use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use crate::{game::GameState, message::WELCOME_MESSAGE};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Option<SubCommands>,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Start a new game session
    Play,
    /// Reset the game progress
    Reset,
}

fn main() -> Result<ExitCode> {
    let args = Args::parse();
    let mut game_state = GameState::load_or_create()?;

    match args.command {
        Some(SubCommands::Play) => {
            println!("{}", WELCOME_MESSAGE);
            game_state.play().context("Failed to start game")?;

            println!("\n👋 Thanks for playing!");
        }
        Some(SubCommands::Reset) => {
            game_state.reset()?;
            println!("Game has been reset.");
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }

    Ok(ExitCode::SUCCESS)
}

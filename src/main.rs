mod crop;
mod game;
mod player;

use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::game::GameState;

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
            game_state.play()?;
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

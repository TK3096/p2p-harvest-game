mod crop;
mod game;
mod player;

use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::game::GameManagement;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Option<SubCommands>,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Start a new game session
    Play,
    /// Check the current game status
    Status,
    /// Reset the game progress
    Reset,
}

fn main() -> Result<ExitCode> {
    let args = Args::parse();

    let mut game = GameManagement::load_or_create()?;

    match args.command {
        Some(SubCommands::Play) => {
            game.run()?;
        }
        Some(SubCommands::Status) => {
            game.display_status()?;
        }
        Some(SubCommands::Reset) => {
            game.reset()?;
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }

    Ok(ExitCode::SUCCESS)
}

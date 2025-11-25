use anyhow::{Context, Result};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use crate::core::GameEngine;

const STATE_FILE: &str = ".game-state.json";

pub struct GamePersistence;

impl GamePersistence {
    /// Load game state from file, or return None if file doesn't exist
    pub fn load() -> Result<Option<GameEngine>> {
        if !Path::new(STATE_FILE).exists() {
            return Ok(None);
        }

        let mut file = File::open(STATE_FILE)
            .with_context(|| format!("Failed to open game state file {}", STATE_FILE))?;

        let mut content = String::new();
        file.read_to_string(&mut content)
            .with_context(|| "Failed to read game state file")?;

        let game_engine: GameEngine =
            serde_json::from_str(&content).with_context(|| "Failed to parse game state file")?;

        Ok(Some(game_engine))
    }

    /// Save game state to file
    pub fn save(game_engine: &GameEngine) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .create(true)
            .open(STATE_FILE)
            .with_context(|| format!("Failed to create/open {}", STATE_FILE))?;

        let json = serde_json::to_string_pretty(game_engine)
            .with_context(|| "Failed to serialize game state")?;

        file.write_all(json.as_bytes())
            .with_context(|| format!("Failed writing file {}", STATE_FILE))?;

        Ok(())
    }

    /// Delete the save file
    pub fn reset() -> Result<()> {
        if Path::new(STATE_FILE).exists() {
            std::fs::remove_file(STATE_FILE)
                .with_context(|| format!("Failed to delete {}", STATE_FILE))?;
        }

        Ok(())
    }

    /// Check if a save file exists
    pub fn save_exists() -> bool {
        Path::new(STATE_FILE).exists()
    }
}

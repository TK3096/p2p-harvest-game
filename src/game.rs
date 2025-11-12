use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::player::Player;

const GAME_STATE_FILE: &str = ".game-state.json";
const INITIAL_DAY: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    pub day: u32,
}

impl GameState {
    fn new(player: Player) -> Self {
        Self {
            player,
            day: INITIAL_DAY,
        }
    }

    pub fn load_or_create() -> Result<Self> {
        if Path::new(GAME_STATE_FILE).exists() {
            let mut file = File::open(GAME_STATE_FILE)
                .with_context(|| format!("Failed to open game state file {}", GAME_STATE_FILE))?;

            let mut content = String::new();
            file.read_to_string(&mut content)
                .with_context(|| format!("Failed to read game state file {}", GAME_STATE_FILE))?;

            let game_state = serde_json::from_str(&content)
                .with_context(|| "Failed to parse game state file")?;

            Ok(game_state)
        } else {
            println!("Name:");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let name = input.trim();
            let player = Player::new(name);

            Ok(Self::new(player))
        }
    }

    fn save(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .truncate(true)
            .open(GAME_STATE_FILE)
            .with_context(|| format!("Failed to create/open {}", GAME_STATE_FILE))?;

        let json =
            serde_json::to_string_pretty(self).with_context(|| "Failed to serialize game state")?;

        file.write_all(json.as_bytes())
            .with_context(|| format!("Failed to write to {}", GAME_STATE_FILE))?;

        Ok(())
    }

    pub fn reset(&self) -> Result<()> {
        if Path::new(GAME_STATE_FILE).exists() {
            std::fs::remove_file(GAME_STATE_FILE)
                .with_context(|| format!("Failed to delete {}", GAME_STATE_FILE))?;
        }
        Ok(())
    }

    pub fn play(&mut self) -> Result<()> {
        loop {
            io::stdout().flush()?;
            println!("plant, sleep, water, harvest, sell, stat, exit");

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let cmd = input.trim();

            match cmd {
                "plant" => {
                    println!("Planting...");
                    if self.player.inventories.is_empty() {
                        println!("No seeds available to plant.");
                        continue;
                    }

                    for (index, seed) in self.player.inventories.iter().enumerate() {
                        println!(
                            "{}: {} (Days to harvest: {})",
                            index + 1,
                            seed.name,
                            seed.days_to_harvest
                        );
                    }

                    println!("Select a seed to plant by number:");
                    io::stdout().flush()?;
                    let mut selection_input = String::new();
                    io::stdin().read_line(&mut selection_input)?;
                    if let Ok(selection) = selection_input.trim().parse::<usize>() {
                        if selection > 0 && selection <= self.player.inventories.len() {
                            let seed = self.player.inventories[selection - 1].clone();
                            match self.player.plant_crop(seed.clone()) {
                                Ok(_) => {
                                    println!("{} planted successfully.", seed.name);
                                }
                                Err(e) => {
                                    println!("Failed to plant {}: {}", seed.name, e);
                                }
                            }
                        } else {
                            println!("Invalid selection.");
                        }
                    } else {
                        println!("Invalid input.");
                    }
                }
                "sleep" => {
                    self.player.sleep();
                    self.day += 1;
                    self.save()?;

                    println!("Sleeping...");
                }
                "water" => {
                    println!("Watering...");
                    match self.player.water_crops(self.day as u8) {
                        Ok(_) => {
                            println!("Crops watered successfully.");
                        }
                        Err(e) => {
                            println!("Failed to water crops: {}", e);
                        }
                    }
                }
                "harvest" => {
                    println!("Harvesting...");

                    match self.player.harvest_crops() {
                        Ok(_) => {
                            println!("Crops harvested successfully.");
                        }
                        Err(e) => {
                            println!("Failed to harvest crops: {}", e);
                        }
                    }
                }
                "sell" => {
                    println!("Selling...");

                    if self.player.warehouses.is_empty() {
                        println!("No crops available to sell.");
                        continue;
                    }

                    for (index, crop) in self.player.warehouses.iter().enumerate() {
                        println!("{}: {} (Price: {})", index + 1, crop.name, crop.price);
                    }

                    println!("Select a crop to sell by number:");
                    io::stdout().flush()?;
                    let mut selection_input = String::new();
                    io::stdin().read_line(&mut selection_input)?;
                    if let Ok(selection) = selection_input.trim().parse::<usize>() {
                        if selection > 0 && selection <= self.player.warehouses.len() {
                            let crop = self.player.warehouses[selection - 1].clone();
                            match self.player.sell_crop(crop.clone()) {
                                Ok(_) => {
                                    println!("{} sold successfully.", crop.name);
                                }
                                Err(e) => {
                                    println!("Failed to sell {}: {}", crop.name, e);
                                }
                            }
                        } else {
                            println!("Invalid selection.");
                        }
                    } else {
                        println!("Invalid input.");
                    }
                }
                "stat" => {
                    println!("Displaying stats");

                    println!("Day: {}", self.day);
                    println!("Player: {:#?}", self.player);
                }
                "exit" => {
                    println!("Exiting game...");
                    break;
                }
                _ => {
                    println!("Unknown command: {}", cmd);
                }
            }
        }

        Ok(())
    }
}

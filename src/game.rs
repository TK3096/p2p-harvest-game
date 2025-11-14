use std::{
    fs::{File, OpenOptions},
    io::{self, Read, StdoutLock, Write},
    path::Path,
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use serde::{Deserialize, Serialize};

use crate::{message, player::Player};

const GAME_STATE_FILE: &str = ".game-state.json";
const INITIAL_DAY: u32 = 1;

#[derive(Debug)]
enum InputEvent {
    Plant,
    Sleep,
    Water,
    Harvest,
    Sell,
    Stat,
    Quit,
}

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
        let mut stdout = io::stdout().lock();
        enable_raw_mode().context("Failed to enable raw mode")?;

        let result = self.run_game_loop(&mut stdout);

        disable_raw_mode().context("Failed to disable raw mode")?;

        result
    }

    pub fn run_game_loop(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        message::display_control_instructions(stdout)?;
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0, 0))?;

        loop {
            if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(event) = event::read() {
                    if let Event::Key(key_event) = event {
                        if let Some(input_event) = Self::handle_key_event(key_event) {
                            match input_event {
                                InputEvent::Quit => {
                                    break;
                                }
                                InputEvent::Plant => {
                                    self.handle_plant_event(stdout)?;
                                }
                                InputEvent::Sleep => {
                                    self.handle_sleep_event(stdout)?;
                                }
                                InputEvent::Water => {
                                    self.handle_water_event(stdout)?;
                                }
                                InputEvent::Harvest => {
                                    self.handle_harvest_event(stdout)?;
                                }
                                InputEvent::Sell => {
                                    self.handle_sell_event(stdout)?;
                                }
                                InputEvent::Stat => {
                                    self.handle_stat_event(stdout)?;
                                }
                                _ => {}
                            }

                            message::display_control_instructions(stdout)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_key_event(key_event: KeyEvent) -> Option<InputEvent> {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => Some(InputEvent::Quit),
            KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(InputEvent::Plant),
            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(InputEvent::Sleep),
            KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(InputEvent::Water),
            KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(InputEvent::Harvest),
            KeyEvent {
                code: KeyCode::Char('l'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(InputEvent::Sell),
            KeyEvent {
                code: KeyCode::Char('i'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(InputEvent::Stat),
            _ => None,
        }
    }

    fn handle_plant_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        if self.player.inventories.is_empty() {
            write!(stdout, "No seeds available to plant.\n")?;
            stdout.flush()?;

            return Ok(());
        }

        Ok(())
    }

    fn handle_sleep_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        write!(stdout, "💤 Sleeping...\r\n")?;
        write!(stdout, "🌞 End of day {}\r\n", self.day)?;

        self.player.sleep();
        self.day += 1;
        self.save()?;

        write!(stdout, "💾 Save completed...\r\n")?;
        stdout.flush()?;

        Ok(())
    }

    fn handle_water_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        Ok(())
    }

    fn handle_harvest_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        Ok(())
    }

    fn handle_sell_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        Ok(())
    }

    fn handle_stat_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        Ok(())
    }

    pub fn play2(&mut self) -> Result<()> {
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

use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
    str::FromStr,
    time::Duration,
};

use crate::player::Player;
use anyhow::{Context, Result};
use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const GAME_STATE_FILE: &str = ".game-state.json";

#[derive(Debug)]
pub enum InputEvent {
    Plant,
    Sleep,
    Water,
    Harvest,
    Status,
    Quit,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GameManagement {
    player: Player,
    day: u32,
}

impl GameManagement {
    pub fn load_or_create() -> Result<Self> {
        if Path::new(GAME_STATE_FILE).exists() {
            let mut file = File::open(GAME_STATE_FILE)
                .with_context(|| format!("Failed to open game state file {}", GAME_STATE_FILE))?;

            let mut content = String::new();
            file.read_to_string(&mut content)
                .with_context(|| "Failed to read game state file")?;

            let game = serde_json::from_str(&content)
                .with_context(|| "Failed to parse game state file")?;

            Ok(game)
        } else {
            println!("Name: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let name = input.trim();

            let game = GameManagement {
                player: Player::new(name),
                day: 1,
            };

            game.save()?;

            Ok(game)
        }
    }

    pub fn save(&self) -> Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(GAME_STATE_FILE)
            .with_context(|| format!("Failed to create/open file {}", GAME_STATE_FILE))?;

        let json =
            serde_json::to_string_pretty(self).with_context(|| "Failed to serialize state")?;

        file.write_all(json.as_bytes())
            .with_context(|| "Failed to write state to file")?;

        Ok(())
    }

    pub fn reset(&self) -> Result<()> {
        if Path::new(GAME_STATE_FILE).exists() {
            std::fs::remove_file(GAME_STATE_FILE).with_context(|| "Failed to remove state file")?;
        }

        println!("Game has been reset.");
        Ok(())
    }

    pub fn display_status(&self) -> Result<()> {
        let mut stdout = io::stdout().lock();

        write!(stdout, "Day: {}\r\n", self.day)?;
        write!(stdout, "Player: {}\r\n", self.player.name)?;
        write!(stdout, "Energy: {}\r\n", self.player.energy)?;
        write!(stdout, "Money: {}\r\n", self.player.money)?;
        write!(stdout, "Inventory:\r\n")?;
        for seed in &self.player.inventory {
            write!(
                stdout,
                "- [{}] {} (Duration: {})\r\n",
                seed.id, seed.name, seed.duration
            )?;
        }
        write!(stdout, "Fields:\r\n")?;
        for crop in &self.player.fields {
            write!(
                stdout,
                "- {} (Watered: {}/{}, Can Harvest: {}, Price: {})\r\n",
                crop.seed.name,
                crop.watered_counts,
                crop.seed.duration,
                crop.can_harvest,
                crop.price
            )?;
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        let mut stdout = io::stdout().lock();

        execute!(stdout, EnterAlternateScreen)?;
        enable_raw_mode().with_context(|| "Failed to enable raw mode")?;

        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0, 0))?;

        loop {
            if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                if let Ok(event) = event::read() {
                    if let Event::Key(key_event) = event {
                        if let Some(input_event) = Self::handle_key_event(key_event) {
                            match input_event {
                                InputEvent::Quit => break,
                                InputEvent::Plant => {
                                    let seeds = self.player.inventory.clone();
                                    let mut selected = 0;

                                    loop {
                                        execute!(stdout, Clear(ClearType::All))?;
                                        execute!(stdout, MoveTo(0, 0))?;
                                        write!(stdout, "Select a seed to plant:\r\n")?;

                                        for (i, seed) in seeds.iter().enumerate() {
                                            if i == selected {
                                                write!(
                                                    stdout,
                                                    "> {} (Duration: {})\r\n",
                                                    seed.name, seed.duration
                                                )?;
                                            } else {
                                                write!(
                                                    stdout,
                                                    "  {} (Duration: {})\r\n",
                                                    seed.name, seed.duration
                                                )?;
                                            }
                                        }

                                        if let Event::Key(key_event) = event::read()? {
                                            match key_event.code {
                                                KeyCode::Up => {
                                                    if selected > 0 {
                                                        selected -= 1;
                                                    }
                                                }
                                                KeyCode::Down => {
                                                    if selected < seeds.len() - 1 {
                                                        selected += 1;
                                                    }
                                                }
                                                KeyCode::Enter => {
                                                    let seed_id = seeds[selected].id;
                                                    self.player.plant(seed_id)?;

                                                    write!(stdout, "Plant success!\r\n")?;
                                                    break;
                                                }
                                                KeyCode::Esc => {
                                                    break;
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                                InputEvent::Sleep => {
                                    self.player.sleep();
                                    self.day += 1;
                                    self.save()?;
                                }
                                InputEvent::Water => {
                                    self.player.water()?;
                                }
                                InputEvent::Harvest => {
                                    self.player.harvest()?;
                                }
                                InputEvent::Status => {
                                    self.display_status()?;
                                }
                            }
                        }
                    }
                }
            }
        }

        disable_raw_mode().with_context(|| "Failed to disable raw mode")?;
        execute!(stdout, LeaveAlternateScreen)?;

        Ok(())
    }

    fn handle_key_event(key_event: KeyEvent) -> Option<InputEvent> {
        match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
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
                code: KeyCode::Char('i'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(InputEvent::Status),
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
            _ => None,
        }
    }
}

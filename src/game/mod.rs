use std::{
    fs::{File, OpenOptions},
    io::{self, Read, StdoutLock, Write},
    path::Path,
};

use anyhow::{Context, Result};
use crossterm::{
    QueueableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};
use serde::{Deserialize, Serialize};

use crate::{event::input::InputEvent, player::Player, trade::TradeNode};

const STATE_FILE: &str = ".game-state.json";
const STARTING_DAY: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    day: u32,
}

impl GameState {
    fn new(player: Player) -> Self {
        Self {
            player,
            day: STARTING_DAY,
        }
    }

    pub fn load_or_create() -> Result<Self> {
        if Path::new(STATE_FILE).exists() {
            let mut file = File::open(STATE_FILE)
                .with_context(|| format!("Failed to open game state file {}", STATE_FILE))?;

            let mut content = String::new();
            file.read_to_string(&mut content)
                .with_context(|| "Failed to read game state file")?;

            let game_state = serde_json::from_str(&content)
                .with_context(|| "Failed to parse game state file")?;

            Ok(game_state)
        } else {
            println!("Your Name: ");
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
            .truncate(true)
            .create(true)
            .open(STATE_FILE)
            .with_context(|| format!("Failed to create/open {}", STATE_FILE))?;

        let json =
            serde_json::to_string_pretty(self).with_context(|| "Failed to serialize game state")?;

        file.write_all(json.as_bytes())
            .with_context(|| format!("Failed writing file {}", STATE_FILE))?;

        Ok(())
    }

    pub fn reset(&self) -> Result<()> {
        if Path::new(STATE_FILE).exists() {
            std::fs::remove_file(STATE_FILE)
                .with_context(|| format!("Failed to delete {}", STATE_FILE))?;
        }

        Ok(())
    }

    pub fn start(&mut self) -> Result<()> {
        let mut stdout = io::stdout().lock();
        let result = self.run_game_loop(&mut stdout);

        result
    }

    pub fn run_game_loop(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        loop {
            write!(stdout, "Control Instructions:\r\n")?;
            write!(
                stdout,
                "🎮 plant/water/harvest/sleep/status/trade/quit 🎮\r\n"
            )?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let cmd = input.trim();
            if let Some(input_event) = InputEvent::from_str(cmd) {
                write!(stdout, "\r\n")?;

                match input_event {
                    InputEvent::Quit => {
                        write!(stdout, "👋 Thanks for playing. Goodbye!\r\n")?;
                        break;
                    }
                    InputEvent::Sleep => {
                        self.handle_sleep(stdout)?;
                    }
                    InputEvent::PlantCrop => {
                        self.handle_plant_crop(stdout)?;
                    }
                    InputEvent::WaterCrops => {
                        self.handle_water_crops(stdout)?;
                    }
                    InputEvent::HarvestCrops => {
                        self.handle_harvest_crops(stdout)?;
                    }
                    InputEvent::Status => {
                        self.display_status(stdout)?;
                    }
                    InputEvent::Trade => {
                        self.handle_trade(stdout)?;
                    }
                }
            } else {
                write!(stdout, "😖 Unknown command: {}\r\n", cmd)?;
            }

            write!(stdout, "\r\n")?;
            write!(stdout, "----------------------------------------\r\n")?;
        }

        Ok(())
    }

    fn handle_sleep(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        write!(stdout, "💤 Good night...\r\n")?;
        write!(stdout, "🌞 End of day {}\r\n", self.day)?;

        self.player.sleep();
        self.day += 1;
        self.save()?;

        write!(stdout, "💾 Save completed...\r\n")?;
        stdout.flush()?;

        Ok(())
    }

    fn handle_plant_crop(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        write!(stdout, "📦 Your inventory:\r\n")?;
        for (index, crop) in self.player.inventory.iter().enumerate() {
            write!(
                stdout,
                "{}. {} (Growth Days: {}, Sell Price: {}, Energy Cost: {})\r\n",
                index + 1,
                crop.name,
                crop.growth_days,
                crop.sell_price,
                crop.energy_cost
            )?;
        }

        write!(stdout, "🌱 Select a crop to plant by number:\r\n")?;
        let mut selected = String::new();
        io::stdin().read_line(&mut selected)?;

        if let Ok(selected) = selected.trim().parse::<usize>() {
            if selected > 0 && selected <= self.player.inventory.len() {
                let crop = self.player.inventory[selected - 1].clone();
                match self.player.plant_crop(crop.clone()) {
                    Ok(_) => {
                        write!(
                            stdout,
                            "🌾 You have planted a {}. Remaining energy: {}\r\n",
                            crop.name, self.player.energy
                        )?;
                    }
                    Err(e) => {
                        write!(stdout, "😖 Failed to plant crop: {}\r\n", e)?;
                    }
                }
            } else {
                write!(stdout, "😖 Invalid selection.")?;
            }
        } else {
            write!(stdout, "😖 Invalid input.\r\n")?;
            return Ok(());
        }

        Ok(())
    }

    fn handle_water_crops(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        match self.player.water_crops(self.day) {
            Ok(_) => {
                write!(
                    stdout,
                    "💧 You have watered your crops. Remaining energy: {}\r\n",
                    self.player.energy
                )?;
            }
            Err(e) => {
                write!(stdout, "😖 Failed to water crops: {}\r\n", e)?;
            }
        }

        Ok(())
    }

    fn handle_harvest_crops(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        match self.player.harvest_crops() {
            Ok(earnings) => {
                write!(
                    stdout,
                    "🌾 You have harvested your crops and earned {} coins! Total money: {}\r\n",
                    earnings, self.player.money
                )?;
            }
            Err(e) => {
                write!(stdout, "😖 Failed to harvest crops: {}\r\n", e)?;
            }
        }

        Ok(())
    }

    fn display_status(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        write!(stdout, "📊 Player Status:\r\n")?;
        write!(stdout, "👤 Name: {}\r\n", self.player.name)?;
        write!(stdout, "🗓️  Day: {}\r\n", self.day)?;
        draw_status_bar(
            stdout,
            "🔋 Energy: ",
            format!("{}/{}", self.player.energy, Player::MAX_ENERGY),
            self.player.energy,
            Player::MAX_ENERGY,
            Color::Green,
            Color::Red,
        )?;
        write!(stdout, "🪙 Money: {}\r\n", self.player.money)?;
        writeln!(stdout)?;

        write!(stdout, "📦 Inventory:\r\n")?;
        if self.player.inventory.is_empty() {
            write!(stdout, "No crops in inventory.\r\n")?;
        } else {
            for crop in &self.player.inventory {
                write!(
                    stdout,
                    "- {} (Growth Days: {}, Sell Price: {})\r\n",
                    crop.name, crop.growth_days, crop.sell_price
                )?;
            }
        }
        writeln!(stdout)?;

        write!(stdout, "🌾 Planted Crops:\r\n")?;
        if self.player.fields.is_empty() {
            write!(stdout, "No crops planted.\r\n")?;
        } else {
            for crop in &self.player.fields {
                draw_status_bar(
                    stdout,
                    &crop.name,
                    if crop.ready_harvest {
                        "Ready to harvest".to_string()
                    } else {
                        format!(
                            "Watered: {}/{} days",
                            crop.watered_days.len(),
                            crop.growth_days
                        )
                    },
                    crop.watered_days.len() as u8,
                    crop.growth_days,
                    Color::Green,
                    Color::Blue,
                )?;
            }
        }

        Ok(())
    }

    fn handle_trade(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        write!(stdout, "🎁 Trade your crops.\r\n")?;
        writeln!(stdout)?;

        write!(stdout, "Select sender/receiver by number\r\n")?;
        write!(stdout, "1. Sedner\r\n")?;
        write!(stdout, "2. Receiver\r\n")?;

        let mut selected = String::new();
        io::stdin().read_line(&mut selected)?;

        if let Ok(selected) = selected.trim().parse::<usize>() {
            if selected == 1 {
                write!(stdout, "You selected Sender.\r\n")?;
                // Implement sender logic here
            } else if selected == 2 {
                write!(stdout, "You selected Receiver.\r\n")?;
                // Implement receiver logic here
            } else {
                write!(stdout, "😖 Invalid selection.")?;
            }
        } else {
            write!(stdout, "😖 Invalid input.\r\n")?;
            return Ok(());
        }

        Ok(())
    }
}

fn draw_status_bar(
    stdout: &mut StdoutLock,
    label: &str,
    surfix_label: String,
    value: u8,
    max_value: u8,
    good_color: Color,
    bad_color: Color,
) -> Result<()> {
    let bar_width = 20;
    let filled = (value as usize * bar_width) / max_value as usize;
    let empty = bar_width - filled;

    write!(stdout, "{}: [", label)?;

    let color = if value > 60 { good_color } else { bad_color };
    stdout.queue(SetForegroundColor(color))?;

    for _ in 0..filled {
        write!(stdout, "█")?;
    }

    stdout.queue(SetForegroundColor(Color::DarkGrey))?;
    for _ in 0..empty {
        write!(stdout, "░")?;
    }

    stdout.queue(ResetColor)?;
    write!(stdout, "] {}\r\n", surfix_label)?;

    Ok(())
}

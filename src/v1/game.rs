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

use crate::{
    message,
    player::{self, Player},
};

const GAME_STATE_FILE: &str = ".game-state.json";
const INITIAL_DAY: u32 = 1;

#[derive(Debug)]
enum InputEvent {
    Plant,
    Sleep,
    Water,
    Harvest,
    Sell,
    Stats,
    Quit,
}

impl InputEvent {
    fn from_string(input: &str) -> Option<Self> {
        match input.to_lowercase().trim() {
            "plant" => Some(InputEvent::Plant),
            "sleep" => Some(InputEvent::Sleep),
            "water" => Some(InputEvent::Water),
            "harvest" => Some(InputEvent::Harvest),
            "sell" => Some(InputEvent::Sell),
            "stats" => Some(InputEvent::Stats),
            "quit" => Some(InputEvent::Quit),
            _ => None,
        }
    }
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
        let result = self.run_game_loop(&mut stdout);

        result
    }

    pub fn run_game_loop(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        loop {
            message::display_control_instructions(stdout)?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let cmd = input.trim();

            if let Some(input_event) = InputEvent::from_string(cmd) {
                write!(stdout, "\r\n")?;
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
                    InputEvent::Stats => {
                        self.handle_stats_event(stdout)?;
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

    fn handle_plant_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        write!(stdout, "📦 Your inventory:\r\n")?;
        for (index, seed) in self.player.inventories.iter().enumerate() {
            write!(
                stdout,
                "{}: {} (Days to harvest: {}) (Sell: {} - {}) (Energy: {})\r\n",
                index + 1,
                seed.name,
                seed.days_to_harvest,
                seed.price_range.0,
                seed.price_range.1,
                seed.energy_required
            )?;
        }

        write!(stdout, "🌱 Select a seed to plant by number:\r\n")?;
        let mut selected = String::new();
        io::stdin().read_line(&mut selected)?;

        if let Ok(selection) = selected.trim().parse::<usize>() {
            if selection > 0 && selection <= self.player.inventories.len() {
                let seed = self.player.inventories[selection - 1].clone();
                match self.player.plant_crop(seed.clone()) {
                    Ok(_) => {
                        write!(stdout, "✅ {} planted successfully.\r\n", seed.name)?;
                    }
                    Err(e) => {
                        write!(stdout, "😖 Failed to plant {}: {}\r\n", seed.name, e)?;
                    }
                }
            } else {
                write!(stdout, "😖 Invalid selection.\r\n")?;
                return Ok(());
            }
        } else {
            write!(stdout, "😖 Invalid input.\r\n")?;
            return Ok(());
        }

        Ok(())
    }

    fn handle_sleep_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        write!(stdout, "💤 Good night...\r\n")?;
        write!(stdout, "🌞 End of day {}\r\n", self.day)?;

        self.player.sleep();
        self.day += 1;
        self.save()?;

        write!(stdout, "💾 Save completed...\r\n")?;
        stdout.flush()?;

        Ok(())
    }

    fn handle_water_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        if self.player.fields.is_empty() {
            write!(stdout, "😖 No crops to water.\r\n")?;
            return Ok(());
        }

        match self.player.water_crops(self.day as u8) {
            Ok(_) => {
                write!(stdout, "✅ Crops watered successfully\r\n")?;
            }
            Err(e) => {
                write!(stdout, "😖 Failed to water crops: {}\r\n", e)?;
            }
        }

        Ok(())
    }

    fn handle_harvest_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        if self.player.fields.is_empty() {
            write!(stdout, "😖 No crops to harvest.\r\n")?;
            return Ok(());
        }

        match self.player.harvest_crops() {
            Ok(_) => {
                write!(stdout, "✅ Crops harvested successfully\r\n")?;
            }
            Err(e) => {
                write!(stdout, "😖 Failed to harvest crops: {}\r\n", e)?;
            }
        }

        Ok(())
    }

    fn handle_sell_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        if self.player.warehouses.is_empty() {
            write!(stdout, "😖 No crops to sell.\r\n")?;
            return Ok(());
        }

        write!(stdout, "📦 Your warehouse:\r\n")?;
        for (index, crop) in self.player.warehouses.iter().enumerate() {
            write!(
                stdout,
                "{}: {} (Price: {})\r\n",
                index + 1,
                crop.name,
                crop.price
            )?;
        }
        write!(stdout, "💰 Select a crop to sell by number:\r\n")?;
        let mut selected = String::new();
        io::stdin().read_line(&mut selected)?;

        if let Ok(selection) = selected.trim().parse::<usize>() {
            if selection > 0 && selection <= self.player.warehouses.len() {
                let crop = self.player.warehouses[selection - 1].clone();
                match self.player.sell_crop(crop.clone()) {
                    Ok(_) => {
                        write!(stdout, "✅ {} sold successfully.\r\n", crop.name)?;
                    }
                    Err(e) => {
                        write!(stdout, "😖 Failed to sell {}: {}\r\n", crop.name, e)?;
                    }
                }
            } else {
                write!(stdout, "😖 Invalid selection.\r\n")?;
                return Ok(());
            }
        } else {
            write!(stdout, "😖 Invalid input.\r\n")?;
            return Ok(());
        }

        Ok(())
    }

    fn handle_stats_event(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        write!(stdout, "📊 Your Stats:\r\n")?;
        writeln!(stdout)?;
        write!(stdout, "🗓️  Day: {}\r\n", self.day)?;
        write!(stdout, "👤 Player: {}\r\n", self.player.name)?;
        write!(stdout, "🪙 Money: {}\r\n", self.player.money)?;
        self.draw_status_bar(
            stdout,
            "🔋 Energy",
            format!("{}/{}", self.player.energy, player::MAX_ENERGY),
            self.player.energy,
            player::MAX_ENERGY,
            Color::Green,
            Color::Red,
        )?;

        writeln!(stdout)?;
        write!(stdout, "📦 Inventory:\r\n")?;
        for seed in &self.player.inventories {
            write!(
                stdout,
                "- {} (Days to harvest: {}) (Sell: {} - {}) (Energy: {})\r\n",
                seed.name,
                seed.days_to_harvest,
                seed.price_range.0,
                seed.price_range.1,
                seed.energy_required
            )?;
        }

        writeln!(stdout)?;
        write!(stdout, "🌱 Field:\r\n")?;
        for crop in &self.player.fields {
            self.draw_status_bar(
                stdout,
                &crop.name,
                if crop.ready_harvest {
                    "Ready to harvest".to_string()
                } else {
                    format!(
                        "Watered: {}/{} days",
                        crop.watered_days.len(),
                        crop.days_to_harvest
                    )
                },
                crop.watered_days.len() as u8,
                crop.days_to_harvest,
                Color::Green,
                Color::Blue,
            )?;
        }

        writeln!(stdout)?;
        write!(stdout, "🚜 Warehouse:\r\n")?;
        for crop in &self.player.warehouses {
            write!(stdout, "- {} (Price: {})\r\n", crop.name, crop.price)?;
        }

        Ok(())
    }

    fn draw_status_bar(
        &self,
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
}

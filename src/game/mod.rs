use std::{
    fs::{File, OpenOptions},
    io::{self, Read, StdoutLock, Write},
    path::Path,
    str::FromStr,
    sync::Arc,
};

use anyhow::{Context, Result};
use chrono::{DateTime, Duration, Utc};
use crossterm::{
    QueueableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};
use iroh::EndpointId;
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, mpsc};

use crate::{
    event::{input::InputEvent, trade::TradeItemType},
    player::Player,
    trade::TradeItem,
    trade_manager::TradeManager,
};

const STATE_FILE: &str = ".game-state.json";
const STARTING_DAY: u32 = 1;
const AUTO_DAY_CHANGE_MINUTES: i64 = 2;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    day: u32,
    #[serde(default)]
    last_day_change: Option<DateTime<Utc>>,
}

impl GameState {
    fn new(player: Player) -> Self {
        Self {
            player,
            day: STARTING_DAY,
            last_day_change: Some(Utc::now()),
        }
    }

    pub fn get_day(&self) -> u32 {
        self.day
    }

    pub fn get_last_day_change(&self) -> Option<DateTime<Utc>> {
        self.last_day_change
    }

    pub fn advance_next_day(&mut self) {
        self.player.sleep();
        self.day += 1;
        self.last_day_change = Some(Utc::now());
    }

    pub fn load_or_create() -> Result<Self> {
        if Path::new(STATE_FILE).exists() {
            let mut file = File::open(STATE_FILE)
                .with_context(|| format!("Failed to open game state file {}", STATE_FILE))?;

            let mut content = String::new();
            file.read_to_string(&mut content)
                .with_context(|| "Failed to read game state file")?;

            let mut game_state: GameState = serde_json::from_str(&content)
                .with_context(|| "Failed to parse game state file")?;

            if game_state.last_day_change.is_none() {
                game_state.last_day_change = Some(Utc::now());
                game_state.save()?;
            }

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

    pub fn save(&self) -> Result<()> {
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
        let mut trade_manager = TradeManager::new()?;
        trade_manager.initialize(self.clone())?;

        let game_state_arc = Arc::new(Mutex::new(self.clone()));

        let (tx, mut rx) = mpsc::unbounded_channel::<u32>();

        let game_state_clone = game_state_arc.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                monitor_day_changes(game_state_clone, tx).await;
            });
        });

        let mut stdout = io::stdout().lock();
        let result = self.run_game_loop(&mut stdout, trade_manager, game_state_arc, &mut rx);

        result
    }

    pub fn run_game_loop(
        &mut self,
        stdout: &mut StdoutLock,
        trade_manager: TradeManager,
        game_state_arc: Arc<Mutex<GameState>>,
        day_rx: &mut mpsc::UnboundedReceiver<u32>,
    ) -> Result<()> {
        loop {
            if let Ok(new_day) = day_rx.try_recv() {
                write!(stdout, "\r\n")?;
                write!(stdout, "⏰ Time has passed! A new day has begun!\r\n")?;
                write!(stdout, "🌞 Welcome to day {}!\r\n", new_day)?;
                write!(stdout, "💤 You feel well rested! Energy restored.\r\n")?;
                write!(stdout, "\r\n")?;
                stdout.flush()?;

                let rt = tokio::runtime::Runtime::new()?;
                *self = rt.block_on(async { game_state_arc.lock().await.clone() });
            }

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

                        let rt = tokio::runtime::Runtime::new()?;
                        rt.block_on(async {
                            *game_state_arc.lock().await = self.clone();
                        });
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
                        self.handle_trade(stdout, &trade_manager)?;
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

        self.advance_next_day();
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

        if let Some(last_change) = self.last_day_change {
            let now = Utc::now();
            let elapsed = now.signed_duration_since(last_change);
            let remaining = Duration::minutes(AUTO_DAY_CHANGE_MINUTES) - elapsed;

            if remaining.num_seconds() > 0 {
                let mins = remaining.num_minutes();
                let secs = remaining.num_seconds() % 60;
                write!(stdout, "⏰ Next auto-day in: {}m {}s\r\n", mins, secs)?;
            } else {
                write!(stdout, "⏰ Next auto-day: Ready!\r\n")?;
            }
        }

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

    fn handle_trade(
        &mut self,
        stdout: &mut StdoutLock,
        trade_manager: &TradeManager,
    ) -> Result<()> {
        write!(stdout, "🎁 P2P Trade System\r\n")?;
        writeln!(stdout)?;

        write!(stdout, "Select mode:\r\n")?;
        write!(
            stdout,
            "1. Send trade (transfer crops/money to another player)\r\n"
        )?;
        write!(stdout, "2. Receive trade (listen for incoming trades)\r\n")?;
        write!(stdout, "3. Cancel\r\n")?;

        let mut selected = String::new();
        io::stdin().read_line(&mut selected)?;

        match selected.trim().parse::<usize>() {
            Ok(1) => {
                self.handle_send_trade(stdout, trade_manager)?;
            }
            Ok(2) => {
                self.handle_receive_trade(stdout, trade_manager)?;
            }
            Ok(3) => {
                write!(stdout, "❌ Trade cancelled.\r\n")?;
            }
            _ => {
                write!(stdout, "😖 Invalid selection.\r\n")?;
            }
        }

        Ok(())
    }

    fn handle_send_trade(
        &mut self,
        stdout: &mut StdoutLock,
        trade_manager: &TradeManager,
    ) -> Result<()> {
        write!(stdout, "\n📤 Send Trade\r\n")?;

        // Get peer endpoint ID
        write!(stdout, "Enter peer's Endpoint ID:\r\n")?;
        let mut endpoint_input = String::new();
        io::stdin().read_line(&mut endpoint_input)?;

        let endpoint_id =
            EndpointId::from_str(endpoint_input.trim()).context("Invalid Endpoint ID format")?;

        // Choose what to send
        write!(stdout, "\nWhat do you want to send?\r\n")?;
        write!(stdout, "1. Money\r\n")?;
        write!(stdout, "2. Crop\r\n")?;

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;

        let trade_item = match choice.trim().parse::<usize>() {
            Ok(1) => {
                // Send money
                write!(stdout, "Enter amount of money to send:\r\n")?;
                let mut amount_input = String::new();
                io::stdin().read_line(&mut amount_input)?;
                let amount = amount_input
                    .trim()
                    .parse::<u32>()
                    .context("Invalid amount")?;

                if self.player.money < amount {
                    write!(
                        stdout,
                        "❌ Not enough money! You have {} coins.\r\n",
                        self.player.money
                    )?;
                    return Ok(());
                }

                TradeItem {
                    item_type: TradeItemType::Money,
                    amount: Some(amount),
                    crop: None,
                }
            }
            Ok(2) => {
                // Send crop
                if self.player.inventory.is_empty() {
                    write!(stdout, "❌ No crops in inventory!\r\n")?;
                    return Ok(());
                }

                write!(stdout, "Your inventory:\r\n")?;
                for (index, crop) in self.player.inventory.iter().enumerate() {
                    write!(stdout, "{}. {}\r\n", index + 1, crop.name)?;
                }

                write!(stdout, "Select crop by number:\r\n")?;
                let mut crop_input = String::new();
                io::stdin().read_line(&mut crop_input)?;
                let crop_index = crop_input
                    .trim()
                    .parse::<usize>()
                    .context("Invalid crop selection")?;

                if crop_index == 0 || crop_index > self.player.inventory.len() {
                    write!(stdout, "❌ Invalid selection!\r\n")?;
                    return Ok(());
                }

                let crop = self.player.inventory[crop_index - 1].clone();

                TradeItem {
                    item_type: TradeItemType::Crop,
                    amount: None,
                    crop: Some(crop),
                }
            }
            _ => {
                write!(stdout, "❌ Invalid choice!\r\n")?;
                return Ok(());
            }
        };

        // Perform the trade
        write!(stdout, "\n📡 Initiating trade...\r\n")?;

        trade_manager.send_trade(endpoint_id, trade_item)?;

        // Reload game state from the shared state in TradeNode
        if let Some(game_state_arc) = trade_manager.get_game_state() {
            let rt = tokio::runtime::Runtime::new()?;
            *self = rt.block_on(async { game_state_arc.lock().await.clone() });
        }

        Ok(())
    }

    fn handle_receive_trade(
        &mut self,
        stdout: &mut StdoutLock,
        trade_manager: &TradeManager,
    ) -> Result<()> {
        write!(stdout, "\n📥 Receive Trade\r\n")?;
        write!(stdout, "Waiting for incoming trades...\r\n")?;
        write!(stdout, "(Will timeout after 60 seconds)\r\n\n")?;

        trade_manager.listen_for_trades(60)?;

        // Reload game state as it may have been updated
        *self = GameState::load_or_create()?;

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

async fn monitor_day_changes(
    game_state: Arc<Mutex<GameState>>,
    day_tx: mpsc::UnboundedSender<u32>,
) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));

    loop {
        interval.tick().await;

        let mut state = game_state.lock().await;

        if let Some(last_change) = state.get_last_day_change() {
            let now = Utc::now();
            let elapsed = now.signed_duration_since(last_change);

            if elapsed >= Duration::minutes(AUTO_DAY_CHANGE_MINUTES) {
                let days_passed = elapsed.num_minutes() / AUTO_DAY_CHANGE_MINUTES;

                for _ in 0..days_passed {
                    state.advance_next_day();
                }

                if let Err(_) = state.save() {
                    eprintln!("Failed to save game state during auto day change.");
                }

                let new_day = state.get_day();

                let _ = day_tx.send(new_day);
            }
        }
    }
}

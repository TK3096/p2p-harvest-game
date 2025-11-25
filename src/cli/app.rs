use anyhow::Result;
use std::io::{self, StdoutLock, Write};
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc};

use crate::core::{
    GameEngine,
    types::{GameCommand, GameResult},
};

use super::{input::InputEvent, persistence::GamePersistence, renderer::GameRenderer};

#[cfg(feature = "network")]
use crate::network::TradeManager;

pub struct CliApp {
    game_engine: GameEngine,
    #[cfg(feature = "network")]
    trade_manager: Option<TradeManager>,
}

impl CliApp {
    pub fn new(game_engine: GameEngine) -> Self {
        Self {
            game_engine,
            #[cfg(feature = "network")]
            trade_manager: None,
        }
    }

    pub fn load_or_create() -> Result<Self> {
        let game_engine = match GamePersistence::load()? {
            Some(engine) => {
                println!("📂 Loaded existing game save");
                engine
            }
            None => {
                println!("🌱 Welcome to the P2P Harvest Game! 🌱");
                println!("Your Name: ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                let name = input.trim();
                GameEngine::new_game(name)
            }
        };

        Ok(Self::new(game_engine))
    }

    pub fn save(&self) -> Result<()> {
        GamePersistence::save(&self.game_engine)
    }

    pub fn reset() -> Result<()> {
        GamePersistence::reset()
    }

    #[cfg(feature = "network")]
    pub fn initialize_networking(&mut self) -> Result<()> {
        let mut trade_manager = TradeManager::new()?;
        trade_manager.initialize(self.game_engine.clone())?;

        let endpoint_id = trade_manager.get_endpoint_id();
        if let Some(id) = endpoint_id {
            println!("🔗 Trade Node initialized!");
            println!("📋 Your Endpoint ID: {}", id);
            println!("Share this ID with other players to trade!\n");
        }

        self.trade_manager = Some(trade_manager);
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        #[cfg(feature = "network")]
        {
            self.initialize_networking()?;
        }

        let game_engine_arc = Arc::new(Mutex::new(self.game_engine.clone()));
        let (tx, mut rx) = mpsc::unbounded_channel();

        // Spawn day change monitor
        let game_engine_clone = game_engine_arc.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                monitor_day_changes(game_engine_clone, tx).await;
            });
        });

        let mut stdout = io::stdout().lock();

        loop {
            // Check for auto day changes
            if let Ok(notification) = rx.try_recv() {
                write!(stdout, "\r\n")?;
                GameRenderer::render_event(&mut stdout, &notification)?;
                write!(stdout, "\r\n")?;
                stdout.flush()?;

                // Sync game engine
                let rt = tokio::runtime::Runtime::new()?;
                self.game_engine = rt.block_on(async { game_engine_arc.lock().await.clone() });
            }

            // Display menu
            write!(stdout, "Control Instructions:\r\n")?;
            write!(
                stdout,
                "🎮 plant/water/harvest/sleep/status/trade/quit 🎮\r\n"
            )?;

            // Read input
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            let cmd = input.trim();
            if let Some(input_event) = InputEvent::from_str(cmd) {
                write!(stdout, "\r\n")?;

                match input_event {
                    InputEvent::Quit => {
                        write!(stdout, "👋 Thanks for playing. Goodbye!\r\n")?;
                        self.save()?;
                        break;
                    }
                    InputEvent::Sleep => {
                        self.handle_sleep(&mut stdout, &game_engine_arc)?;
                    }
                    InputEvent::PlantCrop => {
                        self.handle_plant_crop(&mut stdout)?;
                    }
                    InputEvent::WaterCrops => {
                        self.handle_water_crops(&mut stdout)?;
                    }
                    InputEvent::HarvestCrops => {
                        self.handle_harvest_crops(&mut stdout)?;
                    }
                    InputEvent::Status => {
                        self.handle_status(&mut stdout)?;
                    }
                    InputEvent::Trade => {
                        #[cfg(feature = "network")]
                        self.handle_trade(&mut stdout)?;

                        #[cfg(not(feature = "network"))]
                        write!(
                            stdout,
                            "❌ Trading is not available (network feature disabled)\r\n"
                        )?;
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

    fn handle_sleep(
        &mut self,
        stdout: &mut StdoutLock,
        game_engine_arc: &Arc<Mutex<GameEngine>>,
    ) -> Result<()> {
        let result = self.game_engine.execute(GameCommand::Sleep);

        match result {
            GameResult::Success(event) => {
                GameRenderer::render_event(stdout, &event)?;
                self.save()?;

                // Update shared state
                let rt = tokio::runtime::Runtime::new()?;
                rt.block_on(async {
                    *game_engine_arc.lock().await = self.game_engine.clone();
                });
            }
            GameResult::Error(err) => {
                write!(stdout, "😖 Error: {}\r\n", err)?;
            }
        }

        Ok(())
    }

    fn handle_plant_crop(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        let info = self.game_engine.get_info();

        write!(stdout, "📦 Your inventory:\r\n")?;
        for (index, crop) in info.inventory.iter().enumerate() {
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
            if selected > 0 && selected <= info.inventory.len() {
                let result = self.game_engine.execute(GameCommand::PlantCrop {
                    crop_index: selected - 1,
                });

                match result {
                    GameResult::Success(event) => {
                        GameRenderer::render_event(stdout, &event)?;
                        self.save()?;
                    }
                    GameResult::Error(err) => {
                        write!(stdout, "😖 Failed to plant crop: {}\r\n", err)?;
                    }
                }
            } else {
                write!(stdout, "😖 Invalid selection.\r\n")?;
            }
        } else {
            write!(stdout, "😖 Invalid input.\r\n")?;
        }

        Ok(())
    }

    fn handle_water_crops(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        let result = self.game_engine.execute(GameCommand::WaterCrops);

        match result {
            GameResult::Success(event) => {
                GameRenderer::render_event(stdout, &event)?;
                self.save()?;
            }
            GameResult::Error(err) => {
                write!(stdout, "😖 Failed to water crops: {}\r\n", err)?;
            }
        }

        Ok(())
    }

    fn handle_harvest_crops(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        let result = self.game_engine.execute(GameCommand::HarvestCrops);

        match result {
            GameResult::Success(event) => {
                GameRenderer::render_event(stdout, &event)?;
                self.save()?;
            }
            GameResult::Error(err) => {
                write!(stdout, "😖 Failed to harvest crops: {}\r\n", err)?;
            }
        }

        Ok(())
    }

    fn handle_status(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        let info = self.game_engine.get_info();
        GameRenderer::render_status(stdout, &info)?;
        Ok(())
    }

    #[cfg(feature = "network")]
    fn handle_trade(&mut self, stdout: &mut StdoutLock) -> Result<()> {
        use crate::network::trade_ui;

        if let Some(ref trade_manager) = self.trade_manager {
            trade_ui::handle_trade(stdout, trade_manager, &mut self.game_engine)?;
            self.save()?;
        } else {
            write!(stdout, "❌ Trade manager not initialized\r\n")?;
        }

        Ok(())
    }
}

async fn monitor_day_changes(
    game_engine: Arc<Mutex<GameEngine>>,
    day_tx: mpsc::UnboundedSender<crate::core::types::GameEvent>,
) {
    use chrono::{Duration, Utc};

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(10));

    loop {
        interval.tick().await;

        let mut engine = game_engine.lock().await;
        let config = engine.get_time_config();

        if let Some(last_change) = config.last_day_change {
            let now = Utc::now();
            let elapsed = now.signed_duration_since(last_change);
            let auto_minutes = Duration::minutes(config.auto_day_change_minutes);

            if elapsed >= auto_minutes {
                let days_passed = elapsed.num_minutes() / config.auto_day_change_minutes;

                for _ in 0..days_passed {
                    let result = engine.execute(GameCommand::AdvanceDay);

                    if let GameResult::Success(event) = result {
                        let _ = day_tx.send(event);
                    }
                }
            }
        }
    }
}

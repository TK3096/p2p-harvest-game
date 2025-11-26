use serde::{Deserialize, Serialize};

use super::{
    crop::get_seasonal_crops,
    player::Player,
    season::Season,
    types::{GameCommand, GameEvent, GameInfo, GameResult, SeasonChangeEvent, TimeConfig},
};

const STARTING_DAY: u32 = 1;

/// Core game engine - contains only pure game logic, no I/O operations
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameEngine {
    player: Player,
    day: u32,
    #[serde(default)]
    time_config: TimeConfig,
}

impl GameEngine {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            day: STARTING_DAY,
            time_config: TimeConfig::default(),
        }
    }

    pub fn new_game(player_name: &str) -> Self {
        Self::new(Player::new(player_name))
    }

    /// Execute a game command and return the result
    pub fn execute(&mut self, command: GameCommand) -> GameResult {
        match command {
            GameCommand::Sleep => self.handle_sleep(),
            GameCommand::PlantCrop { crop_index } => self.handle_plant_crop(crop_index),
            GameCommand::WaterCrops => self.handle_water_crops(),
            GameCommand::HarvestCrops => self.handle_harvest_crops(),
            GameCommand::AdvanceDay => self.handle_advance_day(),
            GameCommand::BuySeed { seed_name } => self.handle_buy_seed(seed_name),
        }
    }

    /// Get read-only information about current game state
    pub fn get_info(&self) -> GameInfo {
        let current_season = Season::from_day(self.day);

        GameInfo {
            day: self.day,
            player_name: self.player.name.clone(),
            player_money: self.player.money,
            player_energy: self.player.energy,
            max_energy: Player::MAX_ENERGY,
            current_season,
            year: Season::year(self.day),
            day_in_season: Season::day_in_season(self.day),
            inventory: self.player.inventory.clone(),
            fields: self.player.fields.clone(),
        }
    }

    pub fn get_time_config(&self) -> &TimeConfig {
        &self.time_config
    }

    pub fn set_time_config(&mut self, config: TimeConfig) {
        self.time_config = config;
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn get_day(&self) -> u32 {
        self.day
    }

    pub fn get_current_season(&self) -> Season {
        Season::from_day(self.day)
    }

    // Private command handlers

    fn handle_sleep(&mut self) -> GameResult {
        let old_day = self.day;
        let season_change = self.advance_day();

        GameResult::Success(GameEvent::Slept {
            old_day,
            new_day: self.day,
            season_change,
        })
    }

    fn handle_advance_day(&mut self) -> GameResult {
        let season_change = self.advance_day();

        GameResult::Success(GameEvent::DayAdvanced {
            new_day: self.day,
            season_change,
        })
    }

    fn handle_plant_crop(&mut self, crop_index: usize) -> GameResult {
        if crop_index >= self.player.inventory.len() {
            return GameResult::Error("Invalid crop selection".to_string());
        }

        let crop = self.player.inventory[crop_index].clone();

        match self.player.plant_crop(crop.clone()) {
            Ok(_) => GameResult::Success(GameEvent::CropPlanted {
                crop_name: crop.name,
                remaining_energy: self.player.energy,
            }),
            Err(e) => GameResult::Error(e.to_string()),
        }
    }

    fn handle_water_crops(&mut self) -> GameResult {
        match self.player.water_crops(self.day) {
            Ok(_) => GameResult::Success(GameEvent::CropsWatered {
                remaining_energy: self.player.energy,
            }),
            Err(e) => GameResult::Error(e.to_string()),
        }
    }

    fn handle_harvest_crops(&mut self) -> GameResult {
        match self.player.harvest_crops() {
            Ok(earnings) => GameResult::Success(GameEvent::CropsHarvested {
                earnings,
                total_money: self.player.money,
            }),
            Err(e) => GameResult::Error(e.to_string()),
        }
    }

    fn handle_buy_seed(&mut self, seed_name: String) -> GameResult {
        let current_season = self.get_current_season();
        let available_crops = get_seasonal_crops(current_season);

        // Find the crop by name
        let crop = match available_crops.iter().find(|c| c.name == seed_name) {
            Some(c) => c.clone(),
            None => return GameResult::Error(format!("Seed '{}' not available", seed_name)),
        };

        // Calculate seed cost (50% of sell price)
        let seed_cost = (crop.sell_price as f32 * 0.5) as u32;

        // Check if player has enough money
        if self.player.money < seed_cost {
            return GameResult::Error(format!(
                "Not enough money! Need {} coins, have {}",
                seed_cost, self.player.money
            ));
        }

        // Purchase the seed
        self.player.money -= seed_cost;
        self.player.inventory.push(crop.clone());

        GameResult::Success(GameEvent::SeedPurchased {
            seed_name: crop.name,
            cost: seed_cost,
            remaining_money: self.player.money,
        })
    }

    /// Advance to next day and handle season changes
    fn advance_day(&mut self) -> Option<SeasonChangeEvent> {
        let old_season = self.get_current_season();

        self.player.sleep();
        self.day += 1;
        self.time_config.last_day_change = Some(chrono::Utc::now());

        let new_season = Season::from_day(self.day);

        if old_season != new_season {
            let crops_died = self.handle_season_change(new_season);

            Some(SeasonChangeEvent {
                old_season,
                new_season,
                day: self.day,
                crops_died,
            })
        } else {
            None
        }
    }

    fn handle_season_change(&mut self, new_season: Season) -> Vec<String> {
        let mut died_crops = Vec::new();

        self.player.fields.retain(|crop| {
            if crop.dies_in_season(new_season) {
                died_crops.push(crop.name.clone());
                false
            } else {
                true
            }
        });

        died_crops
    }
}

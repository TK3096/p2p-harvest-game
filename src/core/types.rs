use crate::core::{crop::Crop, season::Season};
use serde::{Deserialize, Serialize};

/// Commands that can be executed on the game engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameCommand {
    Sleep,
    PlantCrop { crop_index: usize },
    WaterCrops,
    HarvestCrops,
    AdvanceDay,
}

/// Result of executing a command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameResult {
    Success(GameEvent),
    Error(String),
}

/// Events that occur as a result of commands or game progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameEvent {
    DayAdvanced {
        new_day: u32,
        season_change: Option<SeasonChangeEvent>,
    },
    Slept {
        old_day: u32,
        new_day: u32,
        season_change: Option<SeasonChangeEvent>,
    },
    CropPlanted {
        crop_name: String,
        remaining_energy: u8,
    },
    CropsWatered {
        remaining_energy: u8,
    },
    CropsHarvested {
        earnings: u32,
        total_money: u32,
    },
    EnergyRestored,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonChangeEvent {
    pub old_season: Season,
    pub new_season: Season,
    pub day: u32,
    pub crops_died: Vec<String>,
}

/// Read-only game state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameInfo {
    pub day: u32,
    pub player_name: String,
    pub player_money: u32,
    pub player_energy: u8,
    pub max_energy: u8,
    pub current_season: Season,
    pub year: u32,
    pub day_in_season: u32,
    pub inventory: Vec<Crop>,
    pub fields: Vec<Crop>,
}

/// Configuration for automatic day progression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeConfig {
    pub auto_day_change_minutes: i64,
    pub last_day_change: Option<chrono::DateTime<chrono::Utc>>,
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            auto_day_change_minutes: 2,
            last_day_change: Some(chrono::Utc::now()),
        }
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::seasson::Season;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crop {
    pub id: Uuid,
    pub name: String,
    pub growth_days: u8,
    pub sell_price: u32,
    pub watered_days: Vec<u32>,
    pub ready_harvest: bool,
    pub energy_cost: u8,
    #[serde(default = "default_seasons")]
    pub seasons: Vec<Season>,
}

fn default_seasons() -> Vec<Season> {
    vec![Season::Spring, Season::Summer, Season::Autumn]
}

impl Crop {
    pub fn new(name: &str, growth_days: u8, sell_price: u32, seasons: Vec<Season>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            growth_days,
            sell_price,
            watered_days: Vec::new(),
            ready_harvest: false,
            energy_cost: 15,
            seasons,
        }
    }

    pub fn can_grow_in_season(&self, season: Season) -> bool {
        self.seasons.contains(&season)
    }

    pub fn dies_in_season(&self, new_season: Season) -> bool {
        !self.can_grow_in_season(new_season)
    }
}

pub fn initiate_starter_crops() -> Vec<Crop> {
    vec![
        Crop::new(
            "Carrot",
            3,
            50,
            vec![Season::Spring, Season::Summer, Season::Autumn],
        ),
        Crop::new("Tomato", 5, 80, vec![Season::Summer]),
        Crop::new("Potato", 4, 60, vec![Season::Spring, Season::Autumn]),
        Crop::new(
            "Wheat",
            7,
            100,
            vec![
                Season::Spring,
                Season::Summer,
                Season::Autumn,
                Season::Winter,
            ],
        ),
    ]
}

pub fn get_seasonal_crops(season: Season) -> Vec<Crop> {
    match season {
        Season::Spring => vec![
            Crop::new(
                "Carrot",
                3,
                50,
                vec![Season::Spring, Season::Summer, Season::Autumn],
            ),
            Crop::new("Potato", 4, 60, vec![Season::Spring, Season::Autumn]),
            Crop::new("Parsnip", 4, 35, vec![Season::Spring]),
        ],
        Season::Summer => vec![
            Crop::new("Tomato", 5, 80, vec![Season::Summer]),
            Crop::new("Corn", 14, 150, vec![Season::Summer, Season::Autumn]),
            Crop::new("Melon", 12, 250, vec![Season::Summer]),
        ],
        Season::Autumn => vec![
            Crop::new("Pumpkin", 13, 320, vec![Season::Autumn]),
            Crop::new("Corn", 14, 150, vec![Season::Summer, Season::Autumn]),
            Crop::new("Yam", 10, 160, vec![Season::Autumn]),
        ],
        Season::Winter => vec![
            Crop::new(
                "Wheat",
                7,
                100,
                vec![
                    Season::Spring,
                    Season::Summer,
                    Season::Autumn,
                    Season::Winter,
                ],
            ),
            Crop::new("Winter Seeds", 7, 80, vec![Season::Winter]),
        ],
    }
}

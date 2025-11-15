use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crop {
    pub id: Uuid,
    pub name: String,
    pub growth_days: u8,
    pub sell_price: u32,
    pub watered_days: Vec<u32>,
    pub ready_harvest: bool,
    pub energy_cost: u8,
}

impl Crop {
    pub fn new(name: &str, growth_days: u8, sell_price: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            growth_days,
            sell_price,
            watered_days: Vec::new(),
            ready_harvest: false,
            energy_cost: 15,
        }
    }
}

pub fn initiate_starter_crops() -> Vec<Crop> {
    vec![
        Crop::new("Carrot", 3, 50),
        Crop::new("Tomato", 5, 80),
        Crop::new("Potato", 4, 60),
    ]
}

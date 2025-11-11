use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seed {
    pub id: Uuid,
    pub name: String,
    pub duration: u16, // watered counts needed to grow
}

impl Seed {
    pub fn new(name: &str, duration: u16) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            duration,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crop {
    pub seed: Seed,
    pub watered_counts: u16,
    pub can_harvest: bool,
    pub is_watered: bool,
    pub price: u32,
}

impl Crop {
    pub fn new(seed: Seed, price: u32) -> Self {
        Self {
            seed,
            is_watered: false,
            watered_counts: 0,
            can_harvest: false,
            price,
        }
    }
}

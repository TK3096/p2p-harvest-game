use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seed {
    pub id: Uuid,
    pub name: String,
    pub growth_count: u32, // number of times watered
}

impl Seed {
    pub fn new(name: String, growth_count: u32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            growth_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crop {
    pub id: Uuid,
    pub seed: Seed,
    pub price: u32,
    pub growth: u32,
    pub can_harvest: bool,
    pub watered_at: Option<u64>,
    pub owner_id: Uuid,
}

impl Crop {
    pub fn new(seed: Seed, owner_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            seed,
            price: 100,
            growth: 0,
            can_harvest: false,
            watered_at: None,
            owner_id,
        }
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seed {
    id: Uuid,
    name: String,
    growth_time: u64, // in hours (milliseconds)
}

impl Seed {
    pub fn new(name: String, growth_time: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            growth_time,
        }
    }
}

// Not done
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crop {
    id: Uuid,
    name: String,
    price: u32,
    growth: u32,
    can_harvest: bool,
    watered_at: u64, // timestamp in milliseconds
    planted_at: u64, // timestamp in milliseconds
}

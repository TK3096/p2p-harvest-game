use std::array;

use chrono::TimeDelta;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::crop::Seed;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    id: Uuid,
    name: String,
    money: u32,
    inventory: [Option<Seed>; 9],
}

impl Default for Player {
    fn default() -> Self {
        let duration = TimeDelta::try_hours(3).unwrap();

        Self {
            id: Uuid::new_v4(),
            name: "Player1".to_string(),
            money: 1000,
            inventory: array::from_fn(|i| {
                if i == 0 {
                    Some(Seed::new(
                        "Turnip".to_string(),
                        duration.num_milliseconds() as u64,
                    ))
                } else {
                    None
                }
            }),
        }
    }
}

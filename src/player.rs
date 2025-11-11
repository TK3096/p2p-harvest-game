use std::array;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::crop::{Crop, Seed};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub money: u32,
    pub inventory: [Option<Seed>; 9],
    pub fields: Vec<Crop>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            money: 1000,
            inventory: array::from_fn(|i| {
                if i == 0 {
                    Some(Seed::new("Turnip".to_string(), 3))
                } else {
                    None
                }
            }),
            fields: Vec::new(),
        }
    }

    pub fn plant(&mut self) {
        let seed = self.inventory[0].as_ref();

        match seed {
            Some(s) => {
                let crop = Crop::new(s.clone(), self.id);

                self.fields.push(crop);

                println!("Planting seed: {}", s.name);
            }
            None => {
                println!("No seed to plant!");
            }
        }
    }

    pub fn status(&self) {
        println!("Player: {}", self.name);
        println!("Money: {}", self.money);
        println!("Inventory:");
        for (i, seed_option) in self.inventory.iter().enumerate() {
            match seed_option {
                Some(seed) => {
                    println!(
                        "  Slot {}: {} (Growth count: {})",
                        i + 1,
                        seed.name,
                        seed.growth_count
                    );
                }
                None => {
                    println!("  Slot {}: Empty", i + 1);
                }
            }
        }
        println!("Fields:");
        for (i, crop) in self.fields.iter().enumerate() {
            println!(
                "  Field {}: {} (Growth: {})",
                i + 1,
                crop.seed.name,
                crop.growth
            );
        }
    }

    pub fn water(&mut self) {
        for crop in &mut self.fields {
            if crop.growth >= crop.seed.growth_count {
                println!("Crop: {} is already fully grown.", crop.seed.name);
                continue;
            }

            crop.growth += 1;
            println!(
                "Watered crop: {}. Growth is now {}",
                crop.seed.name, crop.growth
            );

            if crop.growth == crop.seed.growth_count {
                crop.can_harvest = true;
            }
        }
    }

    pub fn harvest(&mut self) {
        let mut harvested_indices = Vec::new();

        for (i, crop) in self.fields.iter().enumerate() {
            if crop.can_harvest {
                self.money += crop.price;
                println!(
                    "Harvested crop: {} for {} money.",
                    crop.seed.name, crop.price
                );
                harvested_indices.push(i);
            }
        }

        for &index in harvested_indices.iter().rev() {
            self.fields.remove(index);
        }
    }
}

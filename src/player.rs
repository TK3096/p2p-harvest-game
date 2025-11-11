use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::crop::{Crop, Seed};

const MAX_ENERGY: u32 = 100;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub energy: u32,
    pub money: u32,
    pub inventory: Vec<Seed>,
    pub fields: Vec<Crop>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            energy: MAX_ENERGY,
            money: 1000,
            fields: Vec::new(),
            inventory: vec![
                Seed::new("Carrot", 3),
                Seed::new("Tomato", 4),
                Seed::new("Lettuce", 2),
            ],
        }
    }

    pub fn plant(&mut self, seed_id: Uuid) -> Result<()> {
        if self.energy == 0 {
            println!("You don't have enough energy to plant.");
            return Ok(());
        }

        if let Some(seed) = self.inventory.iter().find(|s| s.id == seed_id) {
            let crop = Crop::new(seed.clone(), 100); // Example price
            self.fields.push(crop);
            self.inventory.retain(|s| s.id != seed_id);
        } else {
            println!("Seed not found in your inventory.");
        }

        Ok(())
    }

    pub fn sleep(&mut self) {
        self.energy = MAX_ENERGY;

        self.fields.iter_mut().for_each(|crop| {
            crop.is_watered = false;
        });
    }

    pub fn water(&mut self) -> Result<()> {
        if self.energy == 0 {
            println!("You don't have enough energy to water.");
            return Ok(());
        }

        for crop in self.fields.iter_mut() {
            if !crop.is_watered {
                crop.watered_counts += 1;
                crop.is_watered = true;

                if crop.watered_counts >= crop.seed.duration {
                    crop.can_harvest = true;
                }

                self.energy = self.energy.saturating_sub(10).min(0); // Example energy cost
            }
        }

        Ok(())
    }

    pub fn harvest(&mut self) -> Result<()> {
        let mut total_earnings = 0;

        self.fields.retain(|crop| {
            if crop.can_harvest {
                total_earnings += crop.price;
                false // Remove harvested crop
            } else {
                true // Keep unharvested crop
            }
        });

        self.money += total_earnings;

        Ok(())
    }
}

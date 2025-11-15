use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::crop::{Crop, initiate_starter_crops};

const STARTING_MONEY: u32 = 1000;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub money: u32,
    pub energy: u8,
    pub inventory: Vec<Crop>,
    pub fields: Vec<Crop>,
}

impl Player {
    pub const MAX_ENERGY: u8 = 100;
}

impl Player {
    pub fn new(name: &str) -> Self {
        let starter_crops = initiate_starter_crops();

        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            money: STARTING_MONEY,
            energy: Self::MAX_ENERGY,
            inventory: starter_crops,
            fields: Vec::new(),
        }
    }

    pub fn sleep(&mut self) {
        self.energy = Self::MAX_ENERGY;
    }

    pub fn plant_crop(&mut self, crop: Crop) -> Result<()> {
        if self.energy < crop.energy_cost {
            bail!("Not enough energy to plant the crop");
        }

        self.inventory.retain(|c| c.id != crop.id);
        self.energy = self.energy.saturating_sub(crop.energy_cost).max(0);
        self.fields.push(crop);

        Ok(())
    }

    pub fn water_crops(&mut self, current_day: u32) -> Result<()> {
        if self.fields.is_empty() {
            bail!("No crops to water");
        }

        for crop in &mut self.fields {
            if crop.ready_harvest || crop.watered_days.contains(&current_day) {
                continue;
            }

            if self.energy < crop.energy_cost {
                bail!("Not enough energy to water {}", crop.name);
            }

            crop.watered_days.push(current_day);
            self.energy = self.energy.saturating_sub(crop.energy_cost).max(0);

            if crop.watered_days.len() as u8 == crop.growth_days {
                crop.ready_harvest = true;
            }
        }

        Ok(())
    }

    pub fn harvest_crops(&mut self) -> Result<u32> {
        if self.fields.is_empty() {
            bail!("No crops to harvest");
        }

        let mut total_earnings = 0;

        let mut harvested_crops = Vec::new();
        for crop in &self.fields {
            if crop.ready_harvest {
                total_earnings += crop.sell_price;
                harvested_crops.push(crop.id);
            }
        }

        if total_earnings == 0 {
            bail!("No crops are ready for harvest");
        }

        self.fields.retain(|c| !harvested_crops.contains(&c.id));
        self.money += total_earnings;

        Ok(total_earnings)
    }
}

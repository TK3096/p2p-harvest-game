use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::crop::{Crop, Seed};

const MAX_ENERGY: u8 = 100;
const STARTING_MONEY: u32 = 1000;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
    pub energy: u8,
    pub money: u32,
    pub inventories: Vec<Seed>,
    pub fields: Vec<Crop>,
    pub warehouses: Vec<Crop>,
}

impl Player {
    pub fn new(name: &str) -> Self {
        let starter_seeds = vec![
            Seed::builder()
                .name("Carrot")
                .days_to_harvest(2)
                .price_range((100, 200))
                .energy_required(5)
                .build()
                .unwrap(),
            Seed::builder()
                .name("Tomato")
                .days_to_harvest(6)
                .price_range((150, 250))
                .energy_required(10)
                .build()
                .unwrap(),
        ];

        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            energy: MAX_ENERGY,
            money: STARTING_MONEY,
            inventories: starter_seeds,
            fields: Vec::new(),
            warehouses: Vec::new(),
        }
    }

    pub fn sleep(&mut self) {
        self.energy = MAX_ENERGY;
    }

    pub fn plant_crop(&mut self, seed: Seed) -> Result<()> {
        let crop = Crop::new(seed.clone());
        self.fields.push(crop);
        self.inventories.retain(|s| s.id != seed.id);

        Ok(())
    }

    pub fn water_crops(&mut self, current_day: u8) -> Result<()> {
        for crop in &mut self.fields {
            if self.energy == 0 {
                bail!("You has no energy left!!");
            }

            if self.energy < crop.energy_required {
                bail!("Not enough energy to water the crop: {}", crop.name);
            }

            if crop.ready_harvest || crop.watered_days.contains(&current_day) {
                continue;
            }

            self.energy = self.energy.saturating_sub(crop.energy_required).max(0);
            crop.watered_days.push(current_day);

            if crop.watered_days.len() as u8 == crop.days_to_harvest as u8 {
                crop.ready_harvest = true;
            }
        }

        Ok(())
    }

    pub fn harvest_crops(&mut self) -> Result<()> {
        let mut harvested_crops = Vec::new();

        for crop in &self.fields {
            if crop.ready_harvest {
                self.warehouses.push(crop.clone());
                harvested_crops.push(crop.id);
            }
        }

        self.fields
            .retain(|crop| !harvested_crops.contains(&crop.id));

        Ok(())
    }

    pub fn sell_crop(&mut self, crop: Crop) -> Result<()> {
        if let Some(pos) = self.warehouses.iter().position(|c| c.id == crop.id) {
            self.money += crop.price;
            self.warehouses.remove(pos);
            Ok(())
        } else {
            bail!("{} not found in your warehouse", crop.name);
        }
    }
}

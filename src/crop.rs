use anyhow::{Context, Result};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seed {
    pub id: Uuid,
    pub name: String,
    pub days_to_harvest: u8,
    pub price_range: (u32, u32),
    pub energy_required: u8,
}

pub struct SeedBuilder {
    id: Uuid,
    name: Option<String>,
    days_to_harvest: Option<u8>,
    price_range: Option<(u32, u32)>,
    energy_required: Option<u8>,
}

impl Default for SeedBuilder {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: None,
            days_to_harvest: None,
            price_range: None,
            energy_required: None,
        }
    }
}

impl SeedBuilder {
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn days_to_harvest(mut self, days_to_harvest: u8) -> Self {
        self.days_to_harvest = Some(days_to_harvest);
        self
    }

    pub fn price_range(mut self, price_range: (u32, u32)) -> Self {
        self.price_range = Some(price_range);
        self
    }

    pub fn energy_required(mut self, energy_required: u8) -> Self {
        self.energy_required = Some(energy_required);
        self
    }

    pub fn build(self) -> Result<Seed> {
        Ok(Seed {
            id: self.id,
            name: self.name.context("Seed name is required")?,
            days_to_harvest: self
                .days_to_harvest
                .context("Days to harvest is required")?,
            price_range: self.price_range.context("Price range is required")?,
            energy_required: self
                .energy_required
                .context("Energy required is required")?,
        })
    }
}

impl Seed {
    pub fn builder() -> SeedBuilder {
        SeedBuilder::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crop {
    pub id: Uuid,
    pub name: String,
    pub days_to_harvest: u8,
    pub price: u32,
    pub energy_required: u8,
    pub ready_harvest: bool,
    pub watered_days: Vec<u8>,
}

impl Crop {
    pub fn new(seed: Seed) -> Self {
        let price = rand::random::<u32>() % (seed.price_range.1 - seed.price_range.0 + 1)
            + seed.price_range.0;

        Self {
            id: Uuid::new_v4(),
            name: seed.name,
            days_to_harvest: seed.days_to_harvest,
            price,
            energy_required: seed.energy_required,
            ready_harvest: false,
            watered_days: Vec::new(),
        }
    }
}

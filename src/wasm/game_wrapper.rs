use crate::core::{GameEngine, types::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmGameEngine {
    engine: GameEngine,
}

#[wasm_bindgen]
impl WasmGameEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(player_name: &str) -> Self {
        Self {
            engine: GameEngine::new_game(player_name),
        }
    }

    /// Create a game from JSON state
    #[wasm_bindgen(js_name = fromJson)]
    pub fn from_json(json: &str) -> Result<WasmGameEngine, JsValue> {
        let engine: GameEngine = serde_json::from_str(json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse game state: {}", e)))?;
        Ok(Self { engine })
    }

    /// Export game state as JSON
    #[wasm_bindgen(js_name = toJson)]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.engine)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize game state: {}", e)))
    }

    /// Execute a command and get the result as JSON
    #[wasm_bindgen(js_name = executeCommand)]
    pub fn execute_command(&mut self, command_json: &str) -> String {
        let command: GameCommand = match serde_json::from_str(command_json) {
            Ok(cmd) => cmd,
            Err(e) => {
                return serde_json::to_string(&GameResult::Error(format!(
                    "Invalid command: {}",
                    e
                )))
                .unwrap_or_else(|_| r#"{"Error":"Invalid command"}"#.to_string());
            }
        };

        let result = self.engine.execute(command);
        serde_json::to_string(&result)
            .unwrap_or_else(|_| r#"{"Error":"Serialization failed"}"#.to_string())
    }

    /// Get current game information as JSON
    #[wasm_bindgen(js_name = getInfo)]
    pub fn get_info(&self) -> String {
        let info = self.engine.get_info();
        serde_json::to_string(&info).unwrap_or_else(|_| "{}".to_string())
    }

    // Convenience methods for common operations

    #[wasm_bindgen]
    pub fn sleep(&mut self) -> String {
        let result = self.engine.execute(GameCommand::Sleep);
        serde_json::to_string(&result)
            .unwrap_or_else(|_| r#"{"Error":"Serialization failed"}"#.to_string())
    }

    #[wasm_bindgen(js_name = plantCrop)]
    pub fn plant_crop(&mut self, crop_index: usize) -> String {
        let result = self.engine.execute(GameCommand::PlantCrop { crop_index });
        serde_json::to_string(&result)
            .unwrap_or_else(|_| r#"{"Error":"Serialization failed"}"#.to_string())
    }

    #[wasm_bindgen(js_name = waterCrops)]
    pub fn water_crops(&mut self) -> String {
        let result = self.engine.execute(GameCommand::WaterCrops);
        serde_json::to_string(&result)
            .unwrap_or_else(|_| r#"{"Error":"Serialization failed"}"#.to_string())
    }

    #[wasm_bindgen(js_name = harvestCrops)]
    pub fn harvest_crops(&mut self) -> String {
        let result = self.engine.execute(GameCommand::HarvestCrops);
        serde_json::to_string(&result)
            .unwrap_or_else(|_| r#"{"Error":"Serialization failed"}"#.to_string())
    }

    #[wasm_bindgen(js_name = advanceDay)]
    pub fn advance_day(&mut self) -> String {
        let result = self.engine.execute(GameCommand::AdvanceDay);
        serde_json::to_string(&result)
            .unwrap_or_else(|_| r#"{"Error":"Serialization failed"}"#.to_string())
    }

    // Getters for specific game state

    #[wasm_bindgen(js_name = getDay)]
    pub fn get_day(&self) -> u32 {
        self.engine.get_day()
    }

    #[wasm_bindgen(js_name = getCurrentSeason)]
    pub fn get_current_season(&self) -> String {
        format!("{:?}", self.engine.get_current_season())
    }

    #[wasm_bindgen(js_name = buySeed)]
    pub fn buy_seed(&mut self, seed_name: &str) -> String {
        let result = self.engine.execute(GameCommand::BuySeed {
            seed_name: seed_name.to_string(),
        });
        serde_json::to_string(&result)
            .unwrap_or_else(|_| r#"{"Error":"Serialization failed"}"#.to_string())
    }

    #[wasm_bindgen(js_name = getAvailableSeeds)]
    pub fn get_available_seeds(&self) -> String {
        use crate::core::crop::get_seasonal_crops;
        let current_season = self.engine.get_current_season();
        let available_crops = get_seasonal_crops(current_season);

        // Convert to a simpler format with prices
        let seeds_info: Vec<serde_json::Value> = available_crops
            .iter()
            .map(|crop| {
                let seed_cost = (crop.sell_price as f32 * 0.5) as u32;
                serde_json::json!({
                    "name": crop.name,
                    "cost": seed_cost,
                    "growth_days": crop.growth_days,
                    "sell_price": crop.sell_price,
                    "seasons": crop.seasons,
                    "icon": crop.icon
                })
            })
            .collect();

        serde_json::to_string(&seeds_info).unwrap_or_else(|_| "[]".to_string())
    }
}

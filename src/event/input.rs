#[derive(Debug, PartialEq, Eq)]
pub enum InputEvent {
    Sleep,
    PlantCrop,
    WaterCrops,
    HarvestCrops,
    Status,
    Quit,
}

impl InputEvent {
    pub fn from_str(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            "sleep" | "s" => Some(InputEvent::Sleep),
            "plant" | "p" => Some(InputEvent::PlantCrop),
            "water" | "w" => Some(InputEvent::WaterCrops),
            "harvest" | "h" => Some(InputEvent::HarvestCrops),
            "status" | "i" => Some(InputEvent::Status),
            "quit" | "q" => Some(InputEvent::Quit),
            _ => None,
        }
    }
}

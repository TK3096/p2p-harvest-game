# Migration Guide: Refactored Architecture

## Overview

The codebase has been refactored to separate core game logic from CLI and networking concerns, making it ready for WASM compilation and web UI integration.

## New Structure

```
src/
├── core/                    # Pure game logic (WASM-compatible)
│   ├── mod.rs
│   ├── types.rs            # Game events, commands, and info types
│   ├── game_engine.rs      # Core game engine (no I/O)
│   ├── player.rs           # Player logic
│   ├── crop.rs             # Crop definitions
│   └── season.rs           # Season logic
│
├── cli/                     # Terminal interface
│   ├── mod.rs
│   ├── app.rs              # CLI application logic
│   ├── renderer.rs         # Terminal rendering
│   ├── input.rs            # Input handling
│   └── persistence.rs      # File I/O operations
│
├── network/                 # P2P networking (optional)
│   ├── mod.rs
│   ├── manager.rs          # Trade manager
│   ├── trade_protocol.rs   # Trade protocol implementation
│   └── trade_ui.rs         # Trade UI for CLI
│
├── lib.rs                   # Library exports
└── main.rs                  # CLI entry point
```

## Key Changes

### 1. Core Module (WASM-Compatible)

**GameEngine** (`core/game_engine.rs`):
- Pure game logic with no I/O operations
- Command pattern for all game actions
- Returns events instead of printing directly
- Serializable state

**Usage**:
```rust
use p2p_harvest_game::core::{GameEngine, types::GameCommand};

// Create new game
let mut engine = GameEngine::new_game("PlayerName");

// Execute commands
let result = engine.execute(GameCommand::PlantCrop { crop_index: 0 });

// Get game info
let info = engine.get_info();
println!("Day: {}, Money: {}", info.day, info.player_money);
```

### 2. Event-Driven Architecture

Commands go in, events come out:

```rust
pub enum GameCommand {
    Sleep,
    PlantCrop { crop_index: usize },
    WaterCrops,
    HarvestCrops,
    AdvanceDay,
}

pub enum GameEvent {
    DayAdvanced { new_day: u32, season_change: Option<SeasonChangeEvent> },
    Slept { old_day: u32, new_day: u32, season_change: Option<SeasonChangeEvent> },
    CropPlanted { crop_name: String, remaining_energy: u8 },
    CropsWatered { remaining_energy: u8 },
    CropsHarvested { earnings: u32, total_money: u32 },
}
```

### 3. CLI Module

Handles all terminal I/O:
- **CliApp**: Main CLI application loop
- **GameRenderer**: Renders events to terminal
- **GamePersistence**: File save/load operations
- **InputEvent**: Parses user input

### 4. Network Module

Refactored P2P trading system:
- Works with `GameEngine` instead of `GameState`
- Optional feature flag
- Can be disabled for WASM builds

## Building for Different Targets

### Standard CLI Build (default)
```bash
cargo build --release
```

### CLI Only (no networking)
```bash
cargo build --release --no-default-features --features cli
```

### Core Library Only (WASM-ready)
```bash
cargo build --lib --no-default-features
```

### WASM Build (future)
```bash
# Install wasm-pack first: cargo install wasm-pack
wasm-pack build --target web --no-default-features --features wasm
```

## Using the Core Library in Web

The core library can be used in a web application:

```rust
// In your web UI code (with wasm-bindgen)
use p2p_harvest_game::core::{GameEngine, types::*};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WebGame {
    engine: GameEngine,
}

#[wasm_bindgen]
impl WebGame {
    pub fn new(player_name: &str) -> Self {
        Self {
            engine: GameEngine::new_game(player_name),
        }
    }

    pub fn execute_command(&mut self, command: JsValue) -> JsValue {
        // Parse command from JavaScript
        // Execute on engine
        // Return result as JSON
    }

    pub fn get_info(&self) -> JsValue {
        let info = self.engine.get_info();
        serde_wasm_bindgen::to_value(&info).unwrap()
    }
}
```

## Benefits

1. **Separation of Concerns**: Game logic is independent of UI
2. **Testability**: Core logic can be unit tested without I/O
3. **Reusability**: Same core can power CLI, Web, GUI, or mobile apps
4. **WASM-Ready**: Core module compiles to WebAssembly
5. **Optional Features**: Build only what you need

## Migration from Old Code

### Old Way (GameState):
```rust
// File I/O mixed with game logic
let mut game_state = GameState::load_or_create()?;
game_state.start()?; // Does everything including I/O
```

### New Way (GameEngine + CLI):
```rust
// Separated concerns
let mut app = CliApp::load_or_create()?;  // CLI handles I/O
app.run()?;  // CLI handles rendering and input

// Or use core directly
let mut engine = GameEngine::new_game("Player");
let result = engine.execute(GameCommand::Sleep);
match result {
    GameResult::Success(event) => {
        // Handle event (render, log, etc.)
    }
    GameResult::Error(msg) => {
        // Handle error
    }
}
```

## Testing

### Unit Testing Core Logic
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{GameEngine, types::*};

    #[test]
    fn test_plant_crop() {
        let mut engine = GameEngine::new_game("TestPlayer");
        let result = engine.execute(GameCommand::PlantCrop { crop_index: 0 });
        
        assert!(matches!(result, GameResult::Success(_)));
    }
}
```

## Next Steps

1. Add WASM bindings to `core` module
2. Create web UI using the core library
3. Add more comprehensive tests for core logic
4. Consider adding save/load to core with traits for different storage backends
5. Implement more game features in core (shops, weather, etc.)

## Backwards Compatibility

The old modules (`game`, `player`, `crop`, `seasson`, `trade`, `trade_manager`, `event`) can be safely removed after migration. The new structure provides all the same functionality with better organization.

Save files (`.game-state.json`) are compatible - they will be automatically migrated when loaded by the new `GameEngine`.

## Troubleshooting

### Build Errors

If you encounter errors about missing features:
```bash
cargo clean
cargo build
```

### Feature Flag Issues

Make sure you have the right features enabled:
- For CLI: `--features cli`
- For networking: `--features network`
- For both (default): no flags needed

### WASM Compilation

When compiling to WASM, ensure no CLI or network features are enabled:
```bash
cargo build --lib --target wasm32-unknown-unknown --no-default-features
```

## Support

For questions or issues with the migration, please open an issue on the repository.
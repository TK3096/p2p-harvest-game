# Developer Quick Start Guide

## Overview

This guide will help you understand the refactored architecture and how to work with the codebase.

## Architecture at a Glance

```
┌─────────────────────────────────────────────────────────────┐
│                     Application Layer                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │  CLI App     │  │   Web App    │  │  Mobile App  │      │
│  │  (Terminal)  │  │   (WASM)     │  │   (Native)   │      │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘      │
│         │                  │                  │              │
├─────────┼──────────────────┼──────────────────┼──────────────┤
│         │                  │                  │              │
│         └──────────────────┴──────────────────┘              │
│                            │                                 │
│                    ┌───────▼────────┐                        │
│                    │  Core Engine   │                        │
│                    │  (Pure Logic)  │                        │
│                    └────────────────┘                        │
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Player     │  │    Crops     │  │   Seasons    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

## Module Responsibilities

### Core Module (`src/core/`)

**Purpose**: Pure game logic, no I/O, WASM-compatible

**Files**:
- `game_engine.rs` - Main game engine with command execution
- `player.rs` - Player state and actions
- `crop.rs` - Crop definitions and lifecycle
- `season.rs` - Season calculations
- `types.rs` - Commands, events, and DTOs

**Rules**:
- ❌ No file I/O
- ❌ No stdout/stdin
- ❌ No platform-specific code
- ✅ Pure functions
- ✅ Serializable types
- ✅ Returns events instead of printing

### CLI Module (`src/cli/`)

**Purpose**: Terminal interface implementation

**Files**:
- `app.rs` - Main CLI application loop
- `renderer.rs` - Render events to terminal
- `persistence.rs` - File save/load operations
- `input.rs` - Parse user input

**Rules**:
- ✅ Can use I/O operations
- ✅ Can use platform-specific code
- ✅ Should handle all terminal rendering
- ❌ Should not contain game logic

### Network Module (`src/network/`)

**Purpose**: P2P trading functionality

**Files**:
- `manager.rs` - Async runtime and trade coordination
- `trade_protocol.rs` - Iroh protocol implementation
- `trade_ui.rs` - Trade UI for CLI

**Rules**:
- ✅ Optional via feature flag
- ✅ Can be disabled for WASM builds
- ✅ Works with GameEngine, not directly with game state

## Working with the Core Engine

### Basic Usage

```rust
use p2p_harvest_game::core::{GameEngine, types::*};

// Create a new game
let mut engine = GameEngine::new_game("Alice");

// Execute commands
let result = engine.execute(GameCommand::PlantCrop { crop_index: 0 });

match result {
    GameResult::Success(event) => {
        // Handle the event
        match event {
            GameEvent::CropPlanted { crop_name, remaining_energy } => {
                println!("Planted {}! Energy: {}", crop_name, remaining_energy);
            }
            _ => {}
        }
    }
    GameResult::Error(msg) => {
        eprintln!("Error: {}", msg);
    }
}
```

### Getting Game State

```rust
let info = engine.get_info();
println!("Day: {}", info.day);
println!("Money: {}", info.player_money);
println!("Energy: {}/{}", info.player_energy, info.max_energy);
println!("Season: {:?}", info.current_season);
```

### Available Commands

```rust
pub enum GameCommand {
    Sleep,                         // Advance to next day
    PlantCrop { crop_index: usize }, // Plant a crop from inventory
    WaterCrops,                    // Water all planted crops
    HarvestCrops,                  // Harvest ready crops
    AdvanceDay,                    // Skip to next day (for auto-progression)
}
```

### Event Types

```rust
pub enum GameEvent {
    DayAdvanced { new_day: u32, season_change: Option<SeasonChangeEvent> },
    Slept { old_day: u32, new_day: u32, season_change: Option<SeasonChangeEvent> },
    CropPlanted { crop_name: String, remaining_energy: u8 },
    CropsWatered { remaining_energy: u8 },
    CropsHarvested { earnings: u32, total_money: u32 },
    EnergyRestored,
}
```

## Adding New Features

### Adding a New Game Command

1. **Add to types.rs**:
```rust
pub enum GameCommand {
    // ... existing commands
    BuyCrop { crop_type: String },
}
```

2. **Add event type**:
```rust
pub enum GameEvent {
    // ... existing events
    CropBought { crop_name: String, cost: u32 },
}
```

3. **Implement in game_engine.rs**:
```rust
pub fn execute(&mut self, command: GameCommand) -> GameResult {
    match command {
        // ... existing handlers
        GameCommand::BuyCrop { crop_type } => self.handle_buy_crop(crop_type),
    }
}

fn handle_buy_crop(&mut self, crop_type: String) -> GameResult {
    // Implement logic
    let cost = 100; // Calculate cost
    
    if self.player.money < cost {
        return GameResult::Error("Not enough money".to_string());
    }
    
    // Deduct money and add crop
    self.player.money -= cost;
    // ... add crop to inventory
    
    GameResult::Success(GameEvent::CropBought {
        crop_name: crop_type,
        cost,
    })
}
```

4. **Handle in CLI renderer** (optional):
```rust
impl GameRenderer {
    pub fn render_event(stdout: &mut StdoutLock, event: &GameEvent) -> Result<()> {
        match event {
            // ... existing cases
            GameEvent::CropBought { crop_name, cost } => {
                write!(stdout, "🛒 Bought {} for {} coins!\r\n", crop_name, cost)?;
            }
        }
        stdout.flush()?;
        Ok(())
    }
}
```

### Adding a New Input Command

1. **Add to cli/input.rs**:
```rust
pub enum InputEvent {
    // ... existing
    Shop,
}

impl InputEvent {
    pub fn from_str(input: &str) -> Option<Self> {
        match input.trim().to_lowercase().as_str() {
            // ... existing
            "shop" | "b" => Some(InputEvent::Shop),
            _ => None,
        }
    }
}
```

2. **Handle in cli/app.rs**:
```rust
match input_event {
    // ... existing handlers
    InputEvent::Shop => {
        self.handle_shop(&mut stdout)?;
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
    fn test_plant_crop_success() {
        let mut engine = GameEngine::new_game("TestPlayer");
        let result = engine.execute(GameCommand::PlantCrop { crop_index: 0 });
        
        assert!(matches!(result, GameResult::Success(_)));
        
        if let GameResult::Success(GameEvent::CropPlanted { crop_name, .. }) = result {
            assert_eq!(crop_name, "Carrot");
        }
    }

    #[test]
    fn test_plant_crop_invalid_index() {
        let mut engine = GameEngine::new_game("TestPlayer");
        let result = engine.execute(GameCommand::PlantCrop { crop_index: 999 });
        
        assert!(matches!(result, GameResult::Error(_)));
    }

    #[test]
    fn test_day_progression() {
        let mut engine = GameEngine::new_game("TestPlayer");
        let initial_day = engine.get_day();
        
        engine.execute(GameCommand::Sleep);
        
        assert_eq!(engine.get_day(), initial_day + 1);
    }
}
```

### Integration Testing

```rust
#[test]
fn test_full_crop_lifecycle() {
    let mut engine = GameEngine::new_game("Farmer");
    
    // Plant a crop
    let result = engine.execute(GameCommand::PlantCrop { crop_index: 0 });
    assert!(matches!(result, GameResult::Success(_)));
    
    // Water crops for growth_days
    for _ in 0..3 {
        engine.execute(GameCommand::WaterCrops);
        engine.execute(GameCommand::AdvanceDay);
    }
    
    // Harvest
    let result = engine.execute(GameCommand::HarvestCrops);
    assert!(matches!(result, GameResult::Success(GameEvent::CropsHarvested { .. })));
}
```

## Building for Different Targets

### CLI Application (default)
```bash
cargo build --release
cargo run -- start
```

### Core Library Only
```bash
# Useful for library development
cargo build --lib --no-default-features
```

### Without Networking
```bash
# Faster builds, smaller binary
cargo build --no-default-features --features cli
```

### For WASM (future)
```bash
# Requires wasm-pack: cargo install wasm-pack
wasm-pack build --target web --no-default-features --features wasm
```

## Code Style Guidelines

### Core Module
- Use `Result<T>` for operations that can fail
- Return events instead of side effects
- Keep functions pure when possible
- Document public APIs

### CLI Module
- Handle all I/O in this layer
- Keep rendering logic separate from game logic
- Use `?` operator for error propagation
- Flush stdout after writes

### General
- Use descriptive variable names
- Add doc comments for public types and functions
- Keep functions focused on single responsibility
- Use type aliases for complex types

## Performance Considerations

### Core Engine
- Game state cloning: Used for syncing across threads
- Serialization: Happens on save/load only
- Consider using `Arc` for large shared data

### CLI
- Avoid excessive terminal redraws
- Buffer output when possible
- Use efficient string building

## Debugging Tips

### Enable Logging
```bash
RUST_LOG=debug cargo run -- start
```

### Print Game State
```rust
let info = engine.get_info();
dbg!(info);
```

### Check Serialization
```rust
let json = serde_json::to_string_pretty(&engine)?;
println!("{}", json);
```

### Trace Commands
```rust
let result = engine.execute(command.clone());
println!("Command: {:?} -> Result: {:?}", command, result);
```

## Common Patterns

### Command-Event Pattern
```rust
// Command: What the user wants to do
let command = GameCommand::PlantCrop { crop_index: 0 };

// Execute: Pure logic, no side effects
let result = engine.execute(command);

// Handle Event: Side effects (render, save, log)
match result {
    GameResult::Success(event) => {
        // Render to UI
        GameRenderer::render_event(&mut stdout, &event)?;
        
        // Save state
        GamePersistence::save(&engine)?;
    }
    GameResult::Error(msg) => {
        eprintln!("Error: {}", msg);
    }
}
```

### State Synchronization
```rust
// Share engine across threads
let engine_arc = Arc::new(Mutex::new(engine));

// Clone for other thread
let engine_clone = engine_arc.clone();

// Sync back to main thread
let engine = engine_arc.lock().await.clone();
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/) - Rust fundamentals
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial) - Async programming
- [Serde Guide](https://serde.rs/) - Serialization
- [WASM Book](https://rustwasm.github.io/docs/book/) - WebAssembly with Rust

## Getting Help

1. Check the [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md) for architecture details
2. Read the code documentation: `cargo doc --open`
3. Look at existing implementations in the codebase
4. Open an issue on GitHub

## Next Steps

1. **Explore the codebase**: Start with `src/core/game_engine.rs`
2. **Run the tests**: `cargo test`
3. **Try adding a feature**: Follow the "Adding New Features" section
4. **Build a web UI**: Use the core library with WASM
5. **Contribute**: Submit a pull request!

---

Happy coding! 🚀
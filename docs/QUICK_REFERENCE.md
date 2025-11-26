# Quick Reference Card

## Project Structure

```
src/
├── core/           # Pure game logic (WASM-compatible)
├── cli/            # Terminal interface
├── network/        # P2P networking (optional)
├── lib.rs          # Library exports
└── main.rs         # CLI entry point
```

## Build Commands

```bash
# Full build (CLI + Network)
cargo build --release

# CLI only (no networking)
cargo build --release --no-default-features --features cli

# Core library only (WASM-ready)
cargo build --lib --no-default-features

# Run tests
cargo test

# Run the game
cargo run -- start

# Reset save file
cargo run -- reset
```

## Core API

### Create Game Engine
```rust
use p2p_harvest_game::core::{GameEngine, types::*};

let mut engine = GameEngine::new_game("PlayerName");
```

### Execute Commands
```rust
let result = engine.execute(GameCommand::PlantCrop { crop_index: 0 });

match result {
    GameResult::Success(event) => {
        // Handle event
    }
    GameResult::Error(msg) => {
        // Handle error
    }
}
```

### Get Game Info
```rust
let info = engine.get_info();
println!("Day: {}", info.day);
println!("Money: {}", info.player_money);
println!("Energy: {}/{}", info.player_energy, info.max_energy);
```

## Available Commands

```rust
pub enum GameCommand {
    Sleep,                          // Advance day, restore energy
    PlantCrop { crop_index: usize },// Plant from inventory
    WaterCrops,                     // Water all planted crops
    HarvestCrops,                   // Harvest ready crops
    AdvanceDay,                     // Auto-progression
}
```

## Event Types

```rust
pub enum GameEvent {
    DayAdvanced { new_day, season_change },
    Slept { old_day, new_day, season_change },
    CropPlanted { crop_name, remaining_energy },
    CropsWatered { remaining_energy },
    CropsHarvested { earnings, total_money },
    EnergyRestored,
}
```

## CLI Commands

When running the game:
- `plant` or `p` - Plant a crop
- `water` or `w` - Water crops
- `harvest` or `h` - Harvest crops
- `sleep` or `s` - Sleep (advance day)
- `status` or `i` - View status
- `trade` or `t` - P2P trading
- `quit` or `q` - Quit game

## Module Rules

### Core Module ✅ WASM-Compatible
- ✅ Pure functions
- ✅ Serializable types
- ✅ No I/O operations
- ❌ No file operations
- ❌ No stdout/stdin
- ❌ No platform-specific code

### CLI Module 🖥️ Platform-Specific
- ✅ File I/O allowed
- ✅ Terminal operations
- ✅ User input
- ❌ No game logic

### Network Module 🌐 Optional
- ✅ Feature-gated
- ✅ Works with GameEngine
- ✅ Can be disabled

## Feature Flags

```toml
[features]
default = ["cli", "network"]  # Full app
cli = [...]                   # Terminal UI
network = [...]               # P2P trading
wasm = []                     # Web support
```

## Adding New Features

1. **Add Command** (`core/types.rs`)
```rust
pub enum GameCommand {
    // ...
    NewCommand { params: Type },
}
```

2. **Add Event** (`core/types.rs`)
```rust
pub enum GameEvent {
    // ...
    NewEvent { data: Type },
}
```

3. **Implement Logic** (`core/game_engine.rs`)
```rust
fn handle_new_command(&mut self) -> GameResult {
    // Pure logic
    GameResult::Success(GameEvent::NewEvent { ... })
}
```

4. **Render Event** (`cli/renderer.rs`)
```rust
GameEvent::NewEvent { data } => {
    write!(stdout, "✨ {}\r\n", data)?;
}
```

## Testing

```rust
#[test]
fn test_feature() {
    let mut engine = GameEngine::new_game("Test");
    let result = engine.execute(GameCommand::PlantCrop { crop_index: 0 });
    assert!(matches!(result, GameResult::Success(_)));
}
```

## Common Patterns

### Command Execution
```rust
// Execute command
let result = engine.execute(command);

// Handle result
if let GameResult::Success(event) = result {
    GameRenderer::render_event(&mut stdout, &event)?;
    GamePersistence::save(&engine)?;
}
```

### State Access
```rust
// Read-only access
let info = engine.get_info();

// Mutable access (use carefully)
let player = engine.get_player_mut();
```

### Thread-Safe Sharing
```rust
let engine_arc = Arc::new(Mutex::new(engine));
let clone = engine_arc.clone();
// Use in async/threads
```

## File Locations

- Save file: `.game-state.json`
- Core logic: `src/core/`
- CLI code: `src/cli/`
- Network code: `src/network/`
- Tests: `src/*/tests.rs` or `#[cfg(test)]`

## Documentation

- `README.md` - Project overview
- `MIGRATION_GUIDE.md` - Architecture details
- `DEVELOPER_GUIDE.md` - Detailed guide
- `REFACTORING_SUMMARY.md` - Changes summary
- `QUICK_REFERENCE.md` - This file

## Troubleshooting

### Build Errors
```bash
cargo clean
cargo build
```

### Feature Issues
```bash
# Check which features are enabled
cargo tree -e features
```

### Save File Issues
```bash
# Reset save
cargo run -- reset

# Manually delete
rm .game-state.json
```

## Dependencies

### Core (Always Required)
- serde, serde_json
- uuid, rand, chrono, anyhow

### CLI (Optional)
- tokio, crossterm, clap

### Network (Optional)
- iroh, tokio-stream, async-channel

## Performance Tips

- Use `--release` for production builds
- Disable unused features to reduce binary size
- Core module is lightweight and fast
- CLI rendering can be buffered

## Links

- Rust Book: https://doc.rust-lang.org/book/
- Tokio: https://tokio.rs/
- Serde: https://serde.rs/
- WASM: https://rustwasm.github.io/

## Getting Help

1. Check documentation in this repo
2. Run `cargo doc --open`
3. Read the source code
4. Open an issue on GitHub

---

**Quick Start**: `cargo run -- start` 🚀
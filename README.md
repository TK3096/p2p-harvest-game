# P2P Harvest Game 🌱

A peer-to-peer multiplayer harvest game built with Rust and Iroh networking. Players can grow crops, harvest resources, and interact with each other in a decentralized game environment.

## Features

- 🌾 **Harvest Gameplay**: Plant, grow, and harvest crops
- 🔗 **P2P Networking**: Decentralized multiplayer using Iroh
- 💾 **Persistent State**: Game progress is automatically saved
- 🎮 **Interactive Terminal UI**: Built with crossterm for a smooth CLI experience
- 🚀 **Async Architecture**: Powered by Tokio for efficient networking
- 🌐 **WASM-Ready Core**: Separated core logic can be compiled to WebAssembly
- 🔧 **Modular Design**: Clean separation between game logic, CLI, and networking

## Prerequisites

### For Web UI

1. **Rust** - Required for building WASM
2. **wasm-pack** - Tool for building Rust to WebAssembly
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```
3. **Node.js** (v18+) - For the React frontend
   - Download from: https://nodejs.org/

### For CLI or Development

### Installing Rust

This project requires Rust to build and run. Follow the instructions below for your operating system:

#### macOS, Linux, or Unix-like OS

1. Open a terminal and run:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Follow the on-screen instructions (usually just press Enter to proceed with default installation)

3. After installation, restart your terminal or run:
   ```bash
   source $HOME/.cargo/env
   ```

4. Verify the installation:
   ```bash
   rustc --version
   cargo --version
   ```

#### Windows

1. Download and run the Rust installer from: https://rustup.rs/
2. Follow the installation wizard
3. Restart your terminal/command prompt
4. Verify the installation:
   ```cmd
   rustc --version
   cargo --version
   ```

#### Alternative: Using Package Managers

- **macOS with Homebrew**: `brew install rust`
- **Linux (Arch)**: `pacman -S rust`
- **Linux (Ubuntu/Debian)**: Use rustup method (recommended)

For more details, visit the official Rust installation guide: https://www.rust-lang.org/tools/install

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd p2p-harvest-game
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

### Playing the Game

#### Web UI (Recommended) 🌐

The easiest way to play is through the web interface:

1. Navigate to the web directory:
   ```bash
   cd web
   ```

2. Install dependencies (first time only):
   ```bash
   npm install
   ```

3. Build WASM and start the dev server:
   ```bash
   npm run wasm:dev
   ```

4. Open your browser to `http://localhost:3000`

See [web/README.md](web/README.md) for detailed web UI instructions.

#### Command Line Interface

Start a new game:
```bash
cargo run -- start
```

Or using the release build:
```bash
./target/release/p2p-harvest-game start
```

Reset game state:
```bash
cargo run -- reset
```

**Available Commands:**
- `start` - Start or resume the game
- `reset` - Reset the game state and start fresh

## Development

### Project Structure

The project is now organized into three main modules:

```
p2p-harvest-game/
├── src/
│   ├── core/                # Pure game logic (WASM-compatible)
│   │   ├── game_engine.rs   # Core game engine with no I/O
│   │   ├── player.rs        # Player logic
│   │   ├── crop.rs          # Crop definitions
│   │   ├── season.rs        # Season mechanics
│   │   └── types.rs         # Game commands and events
│   ├── cli/                 # Terminal interface
│   │   ├── app.rs           # CLI application
│   │   ├── renderer.rs      # Terminal rendering
│   │   ├── input.rs         # Input handling
│   │   └── persistence.rs   # File save/load
│   ├── network/             # P2P networking (optional)
│   │   ├── manager.rs       # Trade manager
│   │   ├── trade_protocol.rs
│   │   └── trade_ui.rs
│   ├── lib.rs               # Library exports
│   └── main.rs              # CLI entry point
├── Cargo.toml               # Project dependencies and metadata
├── README.md                # This file
└── MIGRATION_GUIDE.md       # Architecture migration guide
```

**Key Design Principles:**
- **Core Module**: Pure Rust logic with no I/O, ready for WASM compilation
- **CLI Module**: Terminal-specific code (rendering, file I/O, input)
- **Network Module**: Optional P2P functionality with feature flags

### Building for Development

Standard build (includes CLI and networking):
```bash
cargo build
```

Build CLI only (no networking):
```bash
cargo build --no-default-features --features cli
```

Build core library only (WASM-ready):
```bash
cargo build --lib --no-default-features
```

Build for Web (WASM):
```bash
cd web
npm run wasm:build
```

### Running Tests

```bash
cargo test
```

### Running with Verbose Logging

```bash
RUST_LOG=debug cargo run -- start
```

## Architecture

The game uses an **event-driven architecture**:

```rust
// Core game logic (WASM-compatible)
use p2p_harvest_game::core::{GameEngine, types::*};

let mut engine = GameEngine::new_game("PlayerName");
let result = engine.execute(GameCommand::PlantCrop { crop_index: 0 });

match result {
    GameResult::Success(event) => {
        // Handle event (render to UI, log, etc.)
    }
    GameResult::Error(msg) => {
        // Handle error
    }
}
```

**Benefits:**
- Core logic is testable without I/O
- Same core can power CLI, web, GUI, or mobile apps
- Easy to extend with new commands and events

## Dependencies

### Core Dependencies (WASM-compatible)
- **serde/serde_json** - Serialization
- **rand** - Random number generation
- **uuid** - Unique identifiers
- **chrono** - Date and time handling

### CLI Dependencies (optional)
- **tokio** - Async runtime
- **crossterm** - Terminal UI
- **clap** - Command-line argument parsing

### Network Dependencies (optional)
- **iroh** - P2P networking
- **tokio-stream** - Async streams
- **async-channel** - Async channels

## Game State

The game automatically saves your progress to `.game-state.json` in the project directory. This file is created automatically when you first start the game.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

[Add your license information here]

## Troubleshooting

### Rust Edition Error

This project uses Rust edition 2024. Make sure you have an up-to-date version of Rust:

```bash
rustup update
```

### Build Errors

If you encounter build errors, try:

```bash
cargo clean
cargo build
```

### Network Issues

If P2P connectivity is not working, check your firewall settings and ensure the required ports are open.

## Future Development

### Completed ✅
- [x] Refactored architecture with core/CLI separation
- [x] Event-driven game engine
- [x] Feature flags for optional dependencies
- [x] Web UI using React and WASM
- [x] Auto-save functionality (web)
- [x] Responsive design for mobile/desktop

### In Progress 🚧
- [ ] Comprehensive unit tests for core logic
- [ ] Shop system for buying seeds
- [ ] More crop varieties

### Long-term 🎯
- [ ] P2P networking in web UI
- [ ] Enhanced seasons and weather effects
- [ ] Achievements and leaderboards
- [ ] Mobile apps using the same core
- [ ] Multiplayer lobbies and matchmaking
- [ ] Sound effects and animations

## Using the Core Library

The core library can be used independently:

```rust
// Add to Cargo.toml
[dependencies]
p2p-harvest-game = { version = "0.1", default-features = false }

// Use in your code
use p2p_harvest_game::core::{GameEngine, types::*};

let mut engine = GameEngine::new_game("Player");
let info = engine.get_info();
println!("Day: {}, Money: {}", info.day, info.player_money);
```

For WASM usage, see [MIGRATION_GUIDE.md](MIGRATION_GUIDE.md).

---

Built with ❤️ using Rust
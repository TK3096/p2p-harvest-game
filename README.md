# P2P Harvest Game 🌱

A peer-to-peer multiplayer harvest game built with Rust and Iroh networking. Players can grow crops, harvest resources, and interact with each other in a decentralized game environment.

## Features

- 🌾 **Harvest Gameplay**: Plant, grow, and harvest crops
- 🔗 **P2P Networking**: Decentralized multiplayer using Iroh
- 💾 **Persistent State**: Game progress is automatically saved
- 🎮 **Interactive Terminal UI**: Built with crossterm for a smooth CLI experience
- 🚀 **Async Architecture**: Powered by Tokio for efficient networking

## Prerequisites

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

### Start a New Game

```bash
cargo run -- start
```

Or using the release build:
```bash
./target/release/p2p-harvest-game start
```

### Reset Game State

If you want to start fresh and clear your saved progress:

```bash
cargo run -- reset
```

### Available Commands

- `start` - Start or resume the game
- `reset` - Reset the game state and start fresh

## Development

### Project Structure

```
p2p-harvest-game/
├── src/
│   ├── main.rs          # Entry point and CLI interface
│   ├── game/            # Game logic and state management
│   └── ...              # Additional modules
├── Cargo.toml           # Project dependencies and metadata
└── README.md            # This file
```

### Building for Development

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Verbose Logging

```bash
RUST_LOG=debug cargo run -- start
```

## Dependencies

- **tokio** - Async runtime
- **iroh** - P2P networking
- **serde/serde_json** - Serialization
- **crossterm** - Terminal UI
- **clap** - Command-line argument parsing
- **rand** - Random number generation
- **uuid** - Unique identifiers
- **chrono** - Date and time handling

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

## Roadmap

- [ ] Enhanced crop varieties
- [ ] Trading system between players
- [ ] Seasons and weather effects
- [ ] Achievements and leaderboards
- [ ] Cross-platform builds

---

Built with ❤️ using Rust
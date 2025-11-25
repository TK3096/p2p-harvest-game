// Core game logic - WASM compatible
pub mod core;

// CLI interface - requires native features
#[cfg(feature = "cli")]
pub mod cli;

// Network/P2P functionality - requires native features
#[cfg(feature = "network")]
pub mod network;

// Re-export core types for convenience
pub use core::{GameEngine, Player, Season};

// Re-export CLI app when feature is enabled
#[cfg(feature = "cli")]
pub use cli::CliApp;

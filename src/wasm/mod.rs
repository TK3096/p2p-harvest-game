#[cfg(feature = "wasm")]
pub mod game_wrapper;

#[cfg(feature = "wasm")]
pub use game_wrapper::WasmGameEngine;

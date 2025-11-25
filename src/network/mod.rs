#[cfg(feature = "network")]
pub mod manager;
#[cfg(feature = "network")]
pub mod trade_protocol;
#[cfg(feature = "network")]
pub mod trade_ui;

#[cfg(feature = "network")]
pub use manager::TradeManager;

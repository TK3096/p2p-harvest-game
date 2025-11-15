use anyhow::Result;
use std::io::{StdoutLock, Write};

pub const WELCOME_MESSAGE: &str = "🌱 Welcome to the P2P Harvest Game! 🌱";

pub fn display_control_instructions(stdout: &mut StdoutLock) -> Result<()> {
    write!(stdout, "Control Instructions:\r\n")?;
    write!(
        stdout,
        "🎮 plant/water/harvest/sleep/sell/stats/quit 🎮\r\n"
    )?;

    Ok(())
}

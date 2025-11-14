use anyhow::Result;
use std::io::{StdoutLock, Write};

pub const WELCOME_MESSAGE: &str = "🌱 Welcome to the P2P Harvest Game! 🌱";

pub fn display_control_instructions(stdout: &mut StdoutLock) -> Result<()> {
    write!(stdout, "╭─────────────────────────────────╮\r\n")?;
    write!(stdout, "│            CONTROLS             │\r\n")?;
    write!(stdout, "├─────────────────────────────────┤\r\n")?;
    write!(stdout, "│  P       : Plant Crop           │\r\n")?;
    write!(stdout, "│  H       : Harvest Crops        │\r\n")?;
    write!(stdout, "│  W       : Water Crops          │\r\n")?;
    write!(stdout, "│  L       : Sell Crop            │\r\n")?;
    write!(stdout, "│  I       : Display Stat         │\r\n")?;
    write!(stdout, "│  S       : Sleep and Save       │\r\n")?;
    write!(stdout, "│  Q       : Quit Game            │\r\n")?;
    write!(stdout, "╰─────────────────────────────────╯\r\n")?;

    Ok(())
}

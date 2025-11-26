use anyhow::Result;
use crossterm::{
    QueueableCommand,
    style::{Color, ResetColor, SetForegroundColor},
};
use std::io::{StdoutLock, Write};

use crate::core::types::{GameEvent, GameInfo, SeasonChangeEvent};

pub struct GameRenderer;

impl GameRenderer {
    pub fn render_event(stdout: &mut StdoutLock, event: &GameEvent) -> Result<()> {
        match event {
            GameEvent::Slept {
                old_day,
                new_day,
                season_change,
            } => {
                write!(stdout, "💤 Good night...\r\n")?;
                write!(stdout, "🌞 End of day {}\r\n", old_day)?;

                if let Some(change) = season_change {
                    Self::render_season_change(stdout, change)?;
                }

                write!(stdout, "🌞 Welcome to day {}!\r\n", new_day)?;
                write!(stdout, "💤 You feel well rested! Energy restored.\r\n")?;
                write!(stdout, "💾 Save completed...\r\n")?;
            }
            GameEvent::DayAdvanced {
                new_day,
                season_change,
            } => {
                write!(stdout, "⏰ Time has passed! A new day has begun!\r\n")?;
                write!(stdout, "🌞 Welcome to day {}!\r\n", new_day)?;
                write!(stdout, "💤 You feel well rested! Energy restored.\r\n")?;

                if let Some(change) = season_change {
                    Self::render_season_change(stdout, change)?;
                }
            }
            GameEvent::CropPlanted {
                crop_name,
                remaining_energy,
            } => {
                write!(
                    stdout,
                    "🌾 You have planted a {}. Remaining energy: {}\r\n",
                    crop_name, remaining_energy
                )?;
            }
            GameEvent::CropsWatered { remaining_energy } => {
                write!(
                    stdout,
                    "💧 You have watered your crops. Remaining energy: {}\r\n",
                    remaining_energy
                )?;
            }
            GameEvent::CropsHarvested {
                earnings,
                total_money,
            } => {
                write!(
                    stdout,
                    "🌾 You have harvested your crops and earned {} coins! Total money: {}\r\n",
                    earnings, total_money
                )?;
            }
            GameEvent::EnergyRestored => {
                write!(stdout, "⚡ Energy restored!\r\n")?;
            }
            GameEvent::SeedPurchased {
                seed_name,
                cost,
                remaining_money,
            } => {
                write!(
                    stdout,
                    "🏪 Purchased {} seed for {} coins! Remaining money: {}\r\n",
                    seed_name, cost, remaining_money
                )?;
            }
        }

        stdout.flush()?;
        Ok(())
    }

    pub fn render_season_change(stdout: &mut StdoutLock, change: &SeasonChangeEvent) -> Result<()> {
        write!(stdout, "\r\n")?;
        write!(stdout, "🎉 ═══════════════════════════════════ 🎉\r\n")?;
        write!(stdout, "   Season Changed!\r\n")?;
        write!(
            stdout,
            "   {} {} → {} {}\r\n",
            change.old_season.icon(),
            change.old_season.name(),
            change.new_season.icon(),
            change.new_season.name()
        )?;

        if !change.crops_died.is_empty() {
            write!(stdout, "   ❄️  Crops withered:\r\n")?;
            for crop_name in &change.crops_died {
                write!(stdout, "      - {}\r\n", crop_name)?;
            }
        }

        write!(stdout, "🎉 ═══════════════════════════════════ 🎉\r\n")?;
        write!(stdout, "\r\n")?;

        stdout.flush()?;
        Ok(())
    }

    pub fn render_status(stdout: &mut StdoutLock, info: &GameInfo) -> Result<()> {
        write!(stdout, "📊 Player Status:\r\n")?;
        write!(stdout, "👤 Name: {}\r\n", info.player_name)?;
        write!(
            stdout,
            "🗓️  Day: {} (Year: {}, {} Day {})\r\n",
            info.day,
            info.year,
            info.current_season.name(),
            info.day_in_season
        )?;
        write!(stdout, "🌍 Season: {}\r\n", info.current_season)?;

        Self::draw_status_bar(
            stdout,
            "🔋 Energy: ",
            format!("{}/{}", info.player_energy, info.max_energy),
            info.player_energy,
            info.max_energy,
            Color::Green,
            Color::Red,
        )?;
        write!(stdout, "🪙 Money: {}\r\n", info.player_money)?;
        writeln!(stdout)?;

        write!(stdout, "📦 Inventory:\r\n")?;
        if info.inventory.is_empty() {
            write!(stdout, "No crops in inventory.\r\n")?;
        } else {
            for crop in &info.inventory {
                write!(
                    stdout,
                    "- {} {} (Growth Days: {}, Sell Price: {})\r\n",
                    crop.icon, crop.name, crop.growth_days, crop.sell_price
                )?;
            }
        }
        writeln!(stdout)?;

        write!(stdout, "🌾 Planted Crops:\r\n")?;
        if info.fields.is_empty() {
            write!(stdout, "No crops planted.\r\n")?;
        } else {
            for crop in &info.fields {
                Self::draw_status_bar(
                    stdout,
                    &format!("{} {}", crop.icon, crop.name),
                    if crop.ready_harvest {
                        "Ready to harvest".to_string()
                    } else {
                        format!(
                            "Watered: {}/{} days",
                            crop.watered_days.len(),
                            crop.growth_days
                        )
                    },
                    crop.watered_days.len() as u8,
                    crop.growth_days,
                    Color::Green,
                    Color::Blue,
                )?;
            }
        }

        stdout.flush()?;
        Ok(())
    }

    fn draw_status_bar(
        stdout: &mut StdoutLock,
        label: &str,
        suffix_label: String,
        value: u8,
        max_value: u8,
        good_color: Color,
        bad_color: Color,
    ) -> Result<()> {
        let bar_width = 20;
        let filled = (value as usize * bar_width) / max_value as usize;
        let empty = bar_width - filled;

        write!(stdout, "{}: [", label)?;

        let color = if value > 60 { good_color } else { bad_color };
        stdout.queue(SetForegroundColor(color))?;

        for _ in 0..filled {
            write!(stdout, "█")?;
        }

        stdout.queue(SetForegroundColor(Color::DarkGrey))?;
        for _ in 0..empty {
            write!(stdout, "░")?;
        }

        stdout.queue(ResetColor)?;
        write!(stdout, "] {}\r\n", suffix_label)?;

        Ok(())
    }
}

use std::io::{self, Write};

use anyhow::Result;

use crate::player::Player;

#[derive(Debug)]
pub struct Game {
    player: Player,
}

impl Game {
    pub fn new(player_name: String) -> Self {
        Self {
            player: Player::new(player_name),
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        println!("Welcome to the game, {}!", self.player.name);

        loop {
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            match input.trim() {
                "plant" => {
                    self.player.plant();
                }
                "status" => {
                    self.player.status();
                }
                "water" => {
                    self.player.water();
                }
                "harvest" => {
                    self.player.harvest();
                }
                "quit" => {
                    println!("Exiting the game. Goodbye!");
                    break;
                }
                _ => {
                    println!("Unknown command: {}", input.trim());
                }
            }
        }

        Ok(())
    }
}

use crate::player::Player;

mod crop;
mod game;
mod player;

fn main() {
    let player = Player::default();

    println!("Player Info: {:?}", player);
}

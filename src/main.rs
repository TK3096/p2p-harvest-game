use crate::game::Game;

mod crop;
mod game;
mod player;

#[tokio::main]
async fn main() {
    let mut game = Game::new("Player1".to_string());
    game.run().await.unwrap();
}

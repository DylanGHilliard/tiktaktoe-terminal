mod game_manager;
mod game;
mod board;

use game::Game;

use crate::game_manager::GameManager;

fn main() {

    let mut tictak: GameManager = GameManager::new();
    tictak.play_game();
  
}


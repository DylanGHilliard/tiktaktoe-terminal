mod game_manager;
mod game;
mod board;
mod player;
use std::collections::VecDeque;


use player::Player;

use crate::game_manager::GameManager;

fn main() {

    let player1:Player = Player::new("John".to_string(), 'X');
    let player2:Player = Player::new("Paul".to_string(), 'O');
    let mut players:VecDeque<Player> = VecDeque::new();

    players.push_back(player1);
    players.push_back(player2);

    let mut tictak: GameManager = GameManager::new(players);
    tictak.play_game();
  
}


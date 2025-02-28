mod game_manager;
mod game;
mod board;
mod player;
use std::collections::VecDeque;
use std::io::{self, Read};

use player::Player;

use crate::game_manager::GameManager;

fn main() {
    println!("Enter player X name");
    let mut player1_name:String = String::new();
    io::stdin().read_line(&mut player1_name).expect("Failed to read line");

    println!("Enter player O name");
    let mut player2_name:String = String::new();
    io::stdin().read_line(&mut player2_name).expect("Failed to read line");

    let player1:Player = Player::new(player1_name, 'X');
    let player2:Player = Player::new(player2_name, 'O');
    let mut players:VecDeque<Player> = VecDeque::new();

    players.push_back(player1);
    players.push_back(player2);

    let mut tictak: GameManager = GameManager::new(players);

    let mut will_continue: String = 'y'.to_string();

    while will_continue == 'y'.to_string(){
        println!("Start A new Game?\n y for yes\n n for no");
        io::stdin().read_line(&mut will_continue).expect("Not Correct Input");
        
        tictak.play_game();
    }
    
  
}


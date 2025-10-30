mod game_manager;
mod game;
mod board;
mod player;
mod ai;
use std::collections::VecDeque;
use std::io;

use player::Player;

use crate::game_manager::GameManager;

fn main() {



    let players: VecDeque<Player> = player_setup();

    let mut tictak: GameManager = GameManager::new(players);

    let mut can_continue: bool = true;
 

    while can_continue{
        tictak.play_game();
        can_continue = will_continue();
        
    }
    
}

fn will_continue()->bool{
    println!("Do you want to play again? (y/n)");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");
    if response.trim() == "y"{
        return true;
    }
    else if response.trim() == "n"{
        return false;
        
    }
    else{
        println!("Invalid response");
        return will_continue();
    }

}

fn player_setup() -> VecDeque<Player> {

    // Get the player1 name
    println!("Enter player X name");
    let mut player1_name:String = String::new();
    io::stdin().read_line(&mut player1_name).expect("Failed to read line");


    // Get the player2 name
    println!("Enter player O name");
    let mut player2_name:String = String::new();
    io::stdin().read_line(&mut player2_name).expect("Failed to read line");


    // Sets the Player and their symbols
    let player1:Player = Player::new(player1_name, 'X');
    // If the user typed "ai" (case-insensitive) as the second player's name, treat them as an AI
    let player2_trim = player2_name.trim().to_lowercase();
    let player2: Player = if player2_trim == "ai" {
        Player::new_ai("Computer".to_string(), 'O')
    } else {
        Player::new(player2_name, 'O')
    };
    let mut players:VecDeque<Player> = VecDeque::new();

    players.push_back(player1);
    players.push_back(player2);

    return players;
}


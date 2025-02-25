

use std::io;

use crate::board::Board;
use crate::game::Game;


pub struct GameManager {
    pub game: Game,
    pub moves: i32,
}

impl GameManager{
    pub fn new() ->Self{
        let board: Board = Board::new();
        let game: Game = Game::new(board);

        return Self{
            game,
            moves: 0,
        }
    }

    pub fn play_game(&mut self){
        while self.game.status != "COMPLETE"{
           let (mut row, mut col, mut is_valid) = self.get_player_input();

            if !is_valid{
                continue;
            }
            println!("row: {row}   Column: {col} ");

        }
    }

    pub fn get_player_input(&self) -> (i32, i32, bool){
        let mut user_input:String  = String::new();
        
        println!("Enter Row and column with space bettween");
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input.truncate(user_input.len()-1);

        let inputs: Vec<i32> = user_input.split(" ")
        .map(|x| x.parse().expect("Not an Interger!"))
        .collect();;

        if inputs.len() !=2 {
            println!("enter row and column with a space in between");
            return (0, 0, false)
        };

        return (inputs[0], inputs[1], true);


    
    }

}



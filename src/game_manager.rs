
use std::collections::VecDeque;
use std::io;

use crate::board::Board;
use crate::game::Game;
use crate::player::Player;


pub struct GameManager {
    pub game: Game,
    pub moves: i32,
}

impl GameManager{
    pub fn new(_players: VecDeque<Player>) ->Self{
        let board: Board = Board::new();
        let players: VecDeque<Player> = _players;
        let game: Game = Game::new(board, players);

        return Self{
            game,
            moves: 0,
        }
    }

    pub fn play_game(&mut self){
        println!("{}", self.game.board);
        while self.game.status != "COMPLETE"{
           let (mut row, mut col, is_valid) = self.get_player_input();
            row -=1;
            col -=1;
            if !is_valid{
                continue;
            }
            let game_board: &mut Board = &mut self.game.board;


            let is_inserted: bool = game_board.insert_new_symbol(row, col, self.game.players[0].symbol);
            if !is_inserted {
                continue;
            }

            self.moves +=1;
            println!("{}", game_board);

            if self.has_won(row, col, self.game.players[0].symbol) {
                println!("Winner is Player {0} ", self.game.players[0].name);
                self.game.status = "COMPLETE".to_string();
                self.set_score();
                
            }


            self.change_player_turn();
            

        }
    }

    pub fn get_player_input(&self) -> (i32, i32, bool){
        let mut user_input:String  = String::new();
        
        
        println!("Enter Row and column with space bettween");
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        user_input.truncate(user_input.len()-1);

        let inputs: Vec<i32> = user_input.split_whitespace()
        .map(|x| x.parse().expect("Not an Interger!"))
        .collect();

        if inputs.len() !=2 {
            println!("enter row and column with a space in between");
            println!("{}", self.game.board);
            return (0, 0, false)
        };

        return (inputs[0], inputs[1], true);


    
    }

    pub fn change_player_turn(&mut self){
        let tmp_player:Option<Player> = self.game.players.pop_front();
        match tmp_player {
            Some(current_player)=>{
                self.game.players.push_back(current_player);
            },
            _ => ()
        }
        
    }

    pub fn is_row_complete(&mut self, row: i32, symbol: char) ->bool {

        for i in 0..3 {
            if self.game.board.cells[row as usize][i as usize] !=symbol{
                return false
            }
        }
        return true;
    }

    pub fn is_col_complete(&mut self, col: i32, symbol: char) ->bool {

        for i in 0..3 {
            if self.game.board.cells[i as usize][col as usize] !=symbol {
                return false
            }
        }
        return true;
    }

    pub fn is_diagnal_complete(&mut self, symbol: char) ->bool {
        for i in 0..3 {
            if self.game.board.cells [i as usize][i as usize] != symbol {
                return false;
            }
        }
        return true;
    }

    pub fn is_reverse_diagnal_complete(&mut self, symbol: char) ->bool {
        let mut x = 2;
        for i in 0..3 {

            if self.game.board.cells [i as usize][x as usize] != symbol {
                return false;
            }
            x -=1;
        }
        return true;
    }


    pub fn has_won(&mut self, row: i32, col: i32, symbol: char) ->bool {



        if self.is_row_complete(row, symbol) {
            return true;
        }

        if self.is_col_complete(col, symbol) {
            return true;
        }

        if self.is_diagnal_complete(symbol) {
            return true;
        }
        if self.is_reverse_diagnal_complete(symbol) {
            return true;
        }
        return false;

    }

    pub fn set_score(&mut self) {
        self.game.players[0].wins +=1;
        self.game.players[1].losses +=1;
    }

}



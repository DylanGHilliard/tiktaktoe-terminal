use std::collections::VecDeque;
use crate::board::Board;
use crate::player::Player;



pub struct Game{
    pub status: String,
    pub board: Board,
    pub players: VecDeque<Player>,
    
}

impl Game {
    pub fn new(_board:Board, _players: VecDeque<Player>) -> Self
    {
        let board: Board = _board;
        let players:VecDeque<Player> = _players;
        return Self { 
            status: "Not Started".to_string(), 
            board,
            players,
         }
    }



}



use crate::board::Board;


pub struct Game{
    pub status: String,
    pub board: Board
    
}

impl Game {
    pub fn new(board:Board) -> Self
    {
        let board = Board::new();
        return Self { 
            status: "Not Started".to_string(), 
            board,
         }
    }
}


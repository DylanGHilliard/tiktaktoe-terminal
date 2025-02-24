mod board;
use crate::board::Board;

fn main() {

    let board: Board = Board::new();
    
    println!("{:?}", board.cells[0]);
    println!("{:?}", board.cells[1]);
    println!("{:?}", board.cells[2]);
}


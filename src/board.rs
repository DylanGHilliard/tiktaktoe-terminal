use std::{array, fmt};

pub struct Board {
  
    pub cells: Vec<Vec<char>>
}

impl  Board {

    pub fn new() ->Self{
        let cells: Vec<Vec<char>>= vec![vec!['-'; 3];3];

        println!("{:?}", cells);

        return Self{cells}
            
        
    }

    pub fn insert_new_symbol (&mut self, row: i32, col: i32, symbol: char) ->bool {
        if row >= 3 || col >= 3 {
            println!("row and column should be less than 4");
            return false;
        }

        if self.cells[row as usize][col as usize] != '-' {
            println!("Someone already Placed There");
            return false;
        }

        self.cells[row as usize -1][col as usize -1] = symbol;
        return true;
    }

    
}

impl fmt::Display for Board {
    fn fmt(&self, fmt: &mut fmt::Formatter) ->fmt::Result {

        for row in &self.cells {
            for cell in row {
                fmt.write_str(&cell.to_string())?;
                fmt.write_str(" ")?;
            }
            fmt.write_str("\n")?;
        }

        Ok(())
    }
}


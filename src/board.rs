use std::{array, fmt};

pub struct Board {
  
    pub cells: [[char; 3]; 3]
}

impl  Board {

    pub fn new() ->Self{
        let cells: [[char; 3]; 3] = [['-'; 3]; 3];

        println!("{:?}", cells);

        return Self{cells,}
            
        
    }

    
}


pub struct Player{
    pub name: String,
    pub symbol: char,
}

impl Player {
    pub fn new(_name: String, _symbol: char)-> Self{
        let name: String = _name;
        let symbol: char = _symbol;

        return Self {
            name,
            symbol,
        }
        
    }
}
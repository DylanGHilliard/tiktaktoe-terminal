

pub struct Player{
    pub name: String,
    pub symbol: char,
    pub wins: i32,
    pub losses: i32,
}

impl Player {
    pub fn new(_name: String, _symbol: char)-> Self{
        let name: String = _name;
        let symbol: char = _symbol;
        let wins = 0;
        let losses = 0;

        return Self {
            name,
            symbol,
            wins,
            losses
        }
        
    }
}
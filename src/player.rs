

pub struct Player{
    pub name: String,
    pub symbol: char,
    pub wins: i32,
    pub losses: i32,
    pub is_ai: bool,
}

impl Player {
    pub fn new(_name: String, _symbol: char)-> Self{
        let name: String = _name;
        let symbol: char = _symbol;
        let wins = 0;
        let losses = 0;
        let is_ai = false;

        return Self {
            name,
            symbol,
            wins,
            losses,
            is_ai,
        }
    }

    pub fn new_ai(_name: String, _symbol: char) -> Self {
        let mut p = Player::new(_name, _symbol);
        p.is_ai = true;
        return p;
    }
}
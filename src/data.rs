use core::fmt;

pub struct Game {
    pub id: String,
    pub display_name: String,
    pub display_icon: String,
}

impl Game {
    pub const fn new(id: String, display_name: String, display_icon: String) -> Game {
        let game = Game { id, display_name, display_icon };
        return game;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game: {{ display_name: {}, display_icon: {} }}", self.display_name, self.display_icon)
    }
}
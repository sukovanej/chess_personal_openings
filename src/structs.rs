use serde::Deserialize;

#[derive(Debug, PartialEq)]
pub struct Pgn {
    pub date: String,
    pub white: String,
    pub black: String,
    pub result: String,
    pub link: String,
    pub game: PgnGame,
}

impl Pgn {
    pub fn player_won(&self, player: &str) -> bool {
        self.result == "1-0" && self.white == player || self.result == "0-1" && self.black == player
    }
}

pub type PgnGame = Vec<PgnMove>;

#[derive(Debug, PartialEq)]
pub struct PgnMove {
    position: String,
}

impl PgnMove {
    pub fn parse(input: &str) -> Self {
        Self{ position: input.to_string() }
    }
}

#[derive(Deserialize, Debug)]
pub struct ChessApiResponse {
    pub games: Vec<Game>,
}

#[derive(Deserialize, Debug)]
pub struct Game {
    pub pgn: String,
}

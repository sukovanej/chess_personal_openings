use serde::Deserialize;

#[derive(Debug)]
pub struct Pgn {
    pub date: String,
    pub white: String,
    pub black: String,
    pub result: String,
    pub link: String,
    pub game: PgnGame,
}

pub type PgnGame = Vec<PgnMove>;

#[derive(Debug)]
pub struct PgnMove {
    pub position: String,
}

#[derive(Deserialize, Debug)]
pub struct ChessApiResponse {
    pub games: Vec<Game>,
}

#[derive(Deserialize, Debug)]
pub struct Game {
    pub pgn: String,
}
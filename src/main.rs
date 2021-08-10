mod pgn_parse;
mod structs;

use crate::structs::{ChessApiResponse, Pgn};
use crate::pgn_parse::parse_game;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.chess.com/pub/player/sukovanej/games/2021/08")
        .await?
        .json::<ChessApiResponse>()
        .await?;
    println!("{:#?}", resp);
    let games = resp.games.iter().map(parse_game).collect::<Vec<Pgn>>();
    Ok(())
}

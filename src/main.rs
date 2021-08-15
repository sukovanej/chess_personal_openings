mod pgn_parse;
mod structs;
mod openings_finder;

use pgn_parse::parse_game;
use structs::{ChessApiResponse, Pgn};
use openings_finder::find_openings;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.chess.com/pub/player/sukovanej/games/2021/08")
        .await?
        .json::<ChessApiResponse>()
        .await?;

    let games = resp.games.iter().map(parse_game).collect::<Vec<Pgn>>();
    let openings = find_openings(games, 4);

    for opening in openings {
        println!("{:?}", opening);
    }
    Ok(())
}

use crate::structs::{Game, Pgn, PgnGame, PgnMove};

pub fn parse_game(game: &Game) -> Pgn {
    parse_pgn(&game.pgn)
}

fn skip_drop(input: &str, skip: usize, drop: usize) -> String {
    return String::from(&input[skip..input.len() - drop]);
}

pub fn parse_pgn(pgn_input: &String) -> Pgn {
    let metadata_and_moves_input = pgn_input.split("\n\n").collect::<Vec<&str>>();
    let metadata_input = metadata_and_moves_input[0];
    let game_input = metadata_and_moves_input[1];

    let metadata_lines = metadata_input
        .split("\n")
        .map(String::from)
        .collect::<Vec<String>>();

    let date = skip_drop(&metadata_lines[2].trim(), 7, 2);
    let white = skip_drop(&metadata_lines[4].trim(), 8, 2);
    let black = skip_drop(&metadata_lines[5].trim(), 8, 2);
    let result = skip_drop(&metadata_lines[6].trim(), 9, 2);
    let link = skip_drop(&metadata_lines[20].trim(), 7, 2);

    let game = parse_pgn_game(game_input);

    Pgn {
        date,
        white,
        black,
        result,
        link,
        game,
    }
}

fn parse_pgn_game(game_input: &str) -> PgnGame {
    let mut lines: Vec<&str> = vec![];

    let mut tokens = game_input.split(" ");
    let mut moves = vec![];

    loop {
        let number = tokens.next(); // skip number

        let position = tokens.next();

        if position.is_none() {
            break
        }

        moves.push(PgnMove::parse(position.expect("neeee")));
        tokens.next();
        tokens.next();
    }

    return moves;
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PGN: &str = "[Event \"Live Chess\"]\n[Site \"Chess.com\"]\n[Date \"2021.08.01\"]\n[Round \"-\"]\n[White \"benson6753\"]\n[Black \"sukovanej\"] \
    \n[Result \"0-1\"]\n[CurrentPosition \"r4rk1/4pp2/p2p2p1/1pp4p/4PP1P/2PP2P1/PP4K1/R1Bq4 w - -\"] \
    \n[Timezone \"UTC\"]\n[ECO \"B24\"] \
    \n[ECOUrl \"https://www.chess.com/openings/Sicilian-Defense-Closed-Fianchetto-Variation\"]\n[UTCDate \"2021.08.01\"]\n[UTCTime \"11:57:44\"] \
    \n[WhiteElo \"1528\"]\n[BlackElo \"1555\"]\n[TimeControl \"180\"]\n[Termination \"sukovanej won by resignation\"] \
    \n[StartTime \"11:57:44\"]\n[EndDate \"2021.08.01\"]\n[EndTime \"12:00:05\"]\n[Link \"https://www.chess.com/game/live/21558017437\"]\n\n\
    1. e4 {[%clk 0:02:58.5]} 1... c5 {[%clk 0:02:58.8]} 2. Nc3 {[%clk 0:02:57]} 2... Nc6 {[%clk 0:02:57.3]} 3. g3 {[%clk 0:02:56.9]} 3... d6 {[%clk 0:02:56.3]} \
    4. Bg2 {[%clk 0:02:56.8]} 4... Nf6 {[%clk 0:02:53.9]} 5. Nge2 {[%clk 0:02:56.7]} 5... Bg4 {[%clk 0:02:52]} 6. d3 {[%clk 0:02:56.6]} 6... g6 {[%clk 0:02:50.6]} \
    7. O-O {[%clk 0:02:55.4]} 7... Bg7 {[%clk 0:02:49.8]} 8. h3 {[%clk 0:02:54.7]} 8... Bd7 {[%clk 0:02:48]} 9. Kh2 {[%clk 0:02:52.6]} 9... O-O {[%clk 0:02:47]} \
    10. f4 {[%clk 0:02:51]} 10... h5 {[%clk 0:02:46.3]} 11. Ng1 {[%clk 0:02:47.7]} 11... a6 {[%clk 0:02:39.1]} 12. h4 {[%clk 0:02:42.3]} 12... b5 {[%clk 0:02:37.8]} \
    13. Bh3 {[%clk 0:02:40.6]} 13... Ng4+ {[%clk 0:02:32.1]} 14. Kh1 {[%clk 0:02:31.6]} 14... Nd4 {[%clk 0:02:28.7]} 15. Nce2 {[%clk 0:02:26.2]} 15... Qc8 {[%clk 0:02:06]} \
    16. Nxd4 {[%clk 0:02:19.7]} 16... Bxd4 {[%clk 0:02:05.9]} 17. c3 {[%clk 0:02:16.7]} 17... Nf2+ {[%clk 0:01:56.5]} 18. Rxf2 {[%clk 0:02:14.2]} 18... Bxf2 {[%clk 0:01:56.4]} \
    19. Bxd7 {[%clk 0:02:12.4]} 19... Qxd7 {[%clk 0:01:55.5]} 20. Kg2 {[%clk 0:02:10.4]} 20... Bxg1 {[%clk 0:01:49.9]} 21. Kxg1 {[%clk 0:02:07.8]} 21... Qg4 {[%clk 0:01:48.5]} \
    22. Kg2 {[%clk 0:02:06.3]} 22... Qxd1 {[%clk 0:01:47.2]} 0-1\n";

    #[test]
    fn test_pgn_parser() {
        let parsed_game = parse_pgn(&String::from(TEST_PGN));

        assert_eq!(parsed_game.date, "2021.08.01");
        assert_eq!(parsed_game.white, "benson6753");
        assert_eq!(parsed_game.black, "sukovanej");
        assert_eq!(parsed_game.result, "0-1");
        assert_eq!(
            parsed_game.link,
            "https://www.chess.com/game/live/21558017437"
        );

        assert_eq!(parsed_game.player_won("sukovanej"), true);
        assert_eq!(parsed_game.player_won("benson6753"), false);
    }

    #[test]
    fn test_skip_drop() {
        assert_eq!(skip_drop("aaahellobbbb", 3, 4), "hello");
        assert_eq!(skip_drop("aaahellobbbb", 0, 0), "aaahellobbbb");
    }
}

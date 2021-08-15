use std::collections::HashMap;

use crate::structs::{Pgn};

pub fn find_openings(games: Vec<Pgn>, depth: usize) -> Vec<(String, usize)> {
    let mut openings_to_number_of_plays: HashMap<String, usize> = HashMap::new();
    let mut sum = 0;

    for game in games {
        if game.player_won("sukovanej") {
            continue;
        }

        sum += 1;
        let first_n_moves = &game.game[0..depth];
        let first_n_moves_str = first_n_moves.iter().map(|i| i.position.clone()).collect::<Vec<String>>().join(" ");

        match openings_to_number_of_plays.get_mut(&first_n_moves_str) {
            Some(v) => *v += 1,
            None => { openings_to_number_of_plays.insert(first_n_moves_str.clone(), 1); },
        }
    }

    let mut openings_with_number_of_plays: Vec<(String, usize)> = openings_to_number_of_plays.into_iter().collect();
    openings_with_number_of_plays.sort_by_key(|i| i.1);
    openings_with_number_of_plays.reverse();

    println!("celkem {} her", sum);

    openings_with_number_of_plays
}
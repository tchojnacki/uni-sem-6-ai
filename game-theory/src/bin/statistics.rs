use game_theory::{
    ai::{RandomMove, Strategy},
    GameState, Player, BOARD_SQUARES,
};
use std::{cmp::Ordering, collections::HashSet};

const SAMPLE_SIZE: usize = 1_000_000;

fn main() {
    let strategy = RandomMove::default();
    let mut states = HashSet::new();
    states.insert(GameState::reversi_initial());

    while states.len() < SAMPLE_SIZE {
        let mut gs = GameState::reversi_initial();
        while gs.outcome().is_none() {
            gs = gs.make_move(strategy.decide(&gs));
            states.insert(gs.clone());
        }
    }

    let mut total_branches = 0;
    let mut disc_counts_correct = 0;
    let mut occupied_parity_correct = 0;
    let mut round_distribution = [0; BOARD_SQUARES + 1];
    let mut outcome_distribution = [0; BOARD_SQUARES + 1];
    for gs in states.iter() {
        total_branches += gs.move_bitboard().count_ones();

        let occupied_squares = (gs.score_of(Player::Black) + gs.score_of(Player::White)) as usize;
        round_distribution[occupied_squares] += 1;
        if gs.outcome().is_some() {
            outcome_distribution[occupied_squares] += 1;
        }

        let disc_counts_turn = match gs.score_of(Player::Black).cmp(&gs.score_of(Player::White)) {
            Ordering::Less | Ordering::Equal => Player::Black,
            Ordering::Greater => Player::White,
        };

        let occupied_parity_turn = if occupied_squares % 2 == 0 {
            Player::Black
        } else {
            Player::White
        };

        if disc_counts_turn == gs.turn() {
            disc_counts_correct += 1;
        }
        if occupied_parity_turn == gs.turn() {
            occupied_parity_correct += 1;
        }
    }

    println!(
        "Branching factor: {}",
        total_branches as f64 / SAMPLE_SIZE as f64
    );
    println!(
        "Disc counts: {}",
        disc_counts_correct as f64 / SAMPLE_SIZE as f64
    );
    println!(
        "Occupied parity: {}",
        occupied_parity_correct as f64 / SAMPLE_SIZE as f64
    );
    println!(
        "Round distribution:\n{}",
        round_distribution
            .iter()
            .enumerate()
            .map(|(x, y)| format!("({x},{y})"))
            .collect::<Vec<_>>()
            .join("\n")
    );
    println!(
        "Outcome distribution:\n{}",
        outcome_distribution
            .iter()
            .enumerate()
            .map(|(x, y)| format!("({x},{y})"))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

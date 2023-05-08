use game_theory::{
    ai::{RandomMove, Strategy},
    GameState, Player, BOARD_SQUARES,
};
use std::{cmp::Ordering, collections::HashSet};

const SAMPLE_SIZE: usize = 10_000_000;

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

    let mut disc_counts_correct = 0;
    let mut occupied_parity_correct = 0;
    let mut max_branching_factor = 0;
    let mut board_distribution = [0; BOARD_SQUARES + 1];
    let mut branch_distribution = [0; BOARD_SQUARES + 1];
    let mut outcome_distribution = [0; BOARD_SQUARES + 1];
    for gs in states.iter() {
        let occupied_squares = gs.occupied_bb().count_ones() as usize;
        let branching_factor = gs.move_bb().count_ones();

        board_distribution[occupied_squares] += 1;
        branch_distribution[occupied_squares] += branching_factor;
        if gs.outcome().is_some() {
            outcome_distribution[occupied_squares] += 1;
        }

        max_branching_factor = max_branching_factor.max(branching_factor);

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
        "Disc counts: {:.2}%",
        100. * disc_counts_correct as f64 / SAMPLE_SIZE as f64
    );
    println!(
        "Occupied parity: {:.2}%",
        100. * occupied_parity_correct as f64 / SAMPLE_SIZE as f64
    );
    println!("Max branching factor: {max_branching_factor}");
    println!(
        "Average branching factor: {:.2}",
        branch_distribution.iter().sum::<u32>() as f64 / SAMPLE_SIZE as f64
    );
    println!(
        "Expected branching factor: {:.2}",
        branch_distribution
            .iter()
            .enumerate()
            .map(|(x, y)| *y as f64 / board_distribution[x] as f64)
            .sum::<f64>()
            / branch_distribution.len() as f64
    );
    println!(
        "Board distribution:\n{}",
        board_distribution
            .iter()
            .enumerate()
            .map(|(x, y)| format!("({},{})", x as i32 - 3, y))
            .collect::<Vec<_>>()
            .join("\n")
    );
    println!(
        "Branching factor:\n{}",
        branch_distribution
            .iter()
            .take(BOARD_SQUARES)
            .enumerate()
            .map(|(x, y)| format!(
                "({},{:.3})",
                x as i32 - 3,
                *y as f64 / board_distribution[x] as f64
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
    println!(
        "Outcome distribution:\n{}",
        outcome_distribution
            .iter()
            .skip(1)
            .enumerate()
            .map(|(x, y)| format!("({},{})", x as i32 - 3, y))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

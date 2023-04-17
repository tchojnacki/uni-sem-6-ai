use super::strategy::Strategy;
use crate::{GameState, Position};
use rand::{seq::SliceRandom, thread_rng};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

#[derive(Default)]
pub struct ScoreGreedy;

impl Display for ScoreGreedy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(ScoreGreedy))
    }
}

impl Strategy for ScoreGreedy {
    fn decide(&self, gs: &GameState) -> Position {
        let moves = gs.moves();
        let mut best_score = gs.make_move(moves[0]).score_of(gs.turn());
        let mut best_moves = Vec::from([moves[0]]);
        for &position in moves.iter().skip(1) {
            let new_score = gs.make_move(position).score_of(gs.turn());
            match new_score.cmp(&best_score) {
                Ordering::Less => (),
                Ordering::Equal => best_moves.push(position),
                Ordering::Greater => {
                    best_score = new_score;
                    best_moves.clear();
                    best_moves.push(position);
                }
            };
        }
        *best_moves.choose(&mut thread_rng()).unwrap()
    }
}

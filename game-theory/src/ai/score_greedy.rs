use super::strategy::Strategy;
use crate::{GameState, Position};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

pub struct ScoreGreedy {
    rng: StdRng,
}

impl Default for ScoreGreedy {
    fn default() -> Self {
        Self {
            rng: SeedableRng::from_entropy(),
        }
    }
}

impl Display for ScoreGreedy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(ScoreGreedy))
    }
}

impl Strategy for ScoreGreedy {
    fn decide(&mut self, gs: &GameState) -> Position {
        let moves = gs.moves().to_vec();
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
        *best_moves.choose(&mut self.rng).unwrap()
    }
}

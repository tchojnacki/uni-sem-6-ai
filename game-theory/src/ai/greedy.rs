use super::strategy::Strategy;
use crate::{GameState, Position};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

pub struct Greedy {
    rng: StdRng,
}

impl Default for Greedy {
    fn default() -> Self {
        Self {
            rng: SeedableRng::from_entropy(),
        }
    }
}

impl Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(Greedy))
    }
}

impl Strategy for Greedy {
    fn decide(&mut self, gs: &GameState) -> Position {
        let valid_corners = Position::corners()
            .filter(|&c| gs.moves().contains(&c))
            .collect::<Vec<Position>>();
        if !valid_corners.is_empty() {
            return *valid_corners.choose(&mut self.rng).unwrap();
        }

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
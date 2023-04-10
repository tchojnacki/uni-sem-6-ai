use super::minimax::{MAX_PLAYER, MIN_PLAYER};
use crate::{square::Square, GameState, Position, BOARD_SQUARES};
use std::{
    collections::hash_map::DefaultHasher,
    fmt::{self, Display},
    hash::{Hash, Hasher},
};

type WeightMatrix = [i32; BOARD_SQUARES];

#[non_exhaustive]
pub enum Heuristic {
    MaximumDisc,
    MinimumDisc,
    Weighted(Box<WeightMatrix>),
    // Stability,
    // Mobility,
    // FrontierDiscs,
    // CornerCloseness,
    // CornersOwned,
}

impl Display for Heuristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Heuristic::MaximumDisc => write!(f, "MaxD"),
            Heuristic::MinimumDisc => write!(f, "MinD"),
            Heuristic::Weighted(weights) => write!(f, "W{:03}", weights_hash(weights.as_ref())),
        }
    }
}

impl Heuristic {
    pub fn evaluate(&self, gs: &GameState) -> f64 {
        match self {
            Heuristic::MaximumDisc => {
                // Rosenbloom 1982
                gs.score_of(MAX_PLAYER) as f64 - gs.score_of(MIN_PLAYER) as f64
            }
            Heuristic::MinimumDisc => {
                // Stringham 1980
                -&Heuristic::MaximumDisc.evaluate(gs)
            }
            Heuristic::Weighted(weights) => {
                // Maggs 1979
                Position::all()
                    .map(|p| match gs.at(p) {
                        Square::Empty => 0,
                        Square::Placed(MAX_PLAYER) => weights[p.index()],
                        Square::Placed(MIN_PLAYER) => -weights[p.index()],
                    })
                    .sum::<i32>() as f64
            }
        }
    }
}

fn weights_hash(weights: &[i32]) -> u8 {
    let mut hasher = DefaultHasher::new();
    weights.hash(&mut hasher);
    hasher.finish() as u8
}

#[rustfmt::skip]
pub const WEIGHTS_MAGGS: WeightMatrix = [
     64, -30,  10,   5,   5,  10, -30,  64,
    -30, -40,   2,   2,   2,   2, -40, -30,
     10,   2,   5,   1,   1,   5,   2,  10,
      5,   2,   1,   1,   1,   1,   2,   5,
      5,   2,   1,   1,   1,   1,   2,   5,
     10,   2,   5,   1,   1,   5,   2,  10,
    -30, -40,   2,   2,   2,   2, -40, -30,
     64, -30,  10,   5,   5,  10, -30,  64,
];

#[rustfmt::skip]
pub const WEIGHTS_SANNIDHANAM: WeightMatrix = [
     4, -3,  2,  2,  2,  2, -3,  4,
    -3, -4, -1, -1, -1, -1, -4, -3,
     2, -1,  1,  0,  0,  1, -1,  2,
     2, -1,  0,  1,  1,  0, -1,  2,
     2, -1,  0,  1,  1,  0, -1,  2,
     2, -1,  1,  0,  0,  1, -1,  2,
    -3, -4, -1, -1, -1, -1, -4, -3,
     4, -3,  2,  2,  2,  2, -3,  4,
];

#[rustfmt::skip]
pub const WEIGHTS_KORMAN: WeightMatrix = [
    20, -3, 11,  8,  8, 11, -3, 20,
    -3, -7, -4,  1,  1, -4, -7, -3,
    11, -4,  2,  2,  2,  2, -4, 11,
     8,  1,  2, -3, -3,  2,  1,  8,
     8,  1,  2, -3, -3,  2,  1,  8,
    11, -4,  2,  2,  2,  2, -4, 11,
    -3, -7, -4,  1,  1, -4, -7, -3,
    20, -3, 11,  8,  8, 11, -3, 20,
];

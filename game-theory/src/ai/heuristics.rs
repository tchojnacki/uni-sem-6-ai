use super::{
    minimax::{MAX_PLAYER, MIN_PLAYER},
    weights::{weights_hash, WeightMatrix},
};
use crate::{GameState, BOARD_SQUARES};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

#[non_exhaustive]
pub enum Heuristic {
    /// - First mention: Rosenbloom 1982
    /// - AKA: p, coin party, piece difference
    MaximumDisc,
    /// - First mention: Stringham 1980
    MinimumDisc,
    /// - First mention: Maggs 1979
    /// - AKA: d, disk squares, weighted square, static heuristic
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
    #[must_use]
    pub fn evaluate(&self, gs: &GameState) -> f64 {
        match self {
            Heuristic::MaximumDisc => {
                let max = gs.score_of(MAX_PLAYER) as f64;
                let min = gs.score_of(MIN_PLAYER) as f64;
                match max.partial_cmp(&min).unwrap() {
                    Ordering::Less => -min / (min + max),
                    Ordering::Equal => 0.,
                    Ordering::Greater => max / (min + max),
                }
            }
            Heuristic::MinimumDisc => -&Heuristic::MaximumDisc.evaluate(gs),
            Heuristic::Weighted(weights) => {
                let mut max_bb = gs.bitboard(MAX_PLAYER);
                let mut min_bb = gs.bitboard(MIN_PLAYER);
                let mut total = 0.;
                for i in 0..BOARD_SQUARES {
                    total += ((max_bb & 1) as f64 - (min_bb & 1) as f64) * weights[i] as f64;
                    max_bb >>= 1;
                    min_bb >>= 1;
                }
                total
            }
        }
    }
}

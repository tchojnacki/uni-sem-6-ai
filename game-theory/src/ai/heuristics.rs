use super::{
    minimax::{MAX_PLAYER, MIN_PLAYER},
    weights::{weights_hash, WeightMatrix},
};
use crate::{square::Square, GameState, Position};
use std::fmt::{self, Display};

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

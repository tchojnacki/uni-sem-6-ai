use super::minimax::{MAX_PLAYER, MIN_PLAYER};
use crate::GameState;
use std::fmt::{self, Display};

#[non_exhaustive]
pub enum Heuristic {
    MaximumDiscs,
    // MinimumDiscs,
    // Weighted([f64; BOARD_SQUARES]),
    // Stability,
    // Mobility,
    // FrontierDiscs,
    // CornerCloseness,
    // CornersOwned,
}

impl Display for Heuristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Heuristic::MaximumDiscs => write!(f, "MaxD"),
        }
    }
}

impl Heuristic {
    pub fn evaluate(&self, gs: &GameState) -> f64 {
        match self {
            Heuristic::MaximumDiscs => {
                gs.score_of(MAX_PLAYER) as f64 - gs.score_of(MIN_PLAYER) as f64
            }
        }
    }
}

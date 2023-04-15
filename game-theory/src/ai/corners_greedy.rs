use super::strategy::Strategy;
use crate::{
    bitboard::{positions, CORNERS, EMPTY},
    GameState, Position,
};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::fmt::{self, Display};

pub struct CornersGreedy {
    rng: StdRng,
}

impl Default for CornersGreedy {
    fn default() -> Self {
        Self {
            rng: SeedableRng::from_entropy(),
        }
    }
}

impl Display for CornersGreedy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(CornersGreedy))
    }
}

impl Strategy for CornersGreedy {
    fn decide(&mut self, gs: &GameState) -> Position {
        let valid_moves = gs.move_bitboard();
        let valid_corners = valid_moves & CORNERS;

        *positions(if valid_corners != EMPTY {
            valid_corners
        } else {
            valid_moves
        })
        .choose(&mut self.rng)
        .unwrap()
    }
}

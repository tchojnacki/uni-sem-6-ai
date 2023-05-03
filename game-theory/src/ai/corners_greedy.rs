use super::strategy::Strategy;
use crate::{bitboard as bb, GameState, Position};
use rand::{seq::SliceRandom, thread_rng};
use std::fmt::{self, Display};

#[derive(Default)]
pub struct CornersGreedy;

impl Display for CornersGreedy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(CornersGreedy))
    }
}

impl Strategy for CornersGreedy {
    fn decide(&self, gs: &GameState) -> Position {
        let valid_moves = gs.move_bb();
        let valid_corners = valid_moves & bb::CORNERS;

        *bb::positions(if valid_corners != bb::EMPTY {
            valid_corners
        } else {
            valid_moves
        })
        .choose(&mut thread_rng())
        .unwrap()
    }
}

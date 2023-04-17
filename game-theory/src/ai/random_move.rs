use super::strategy::Strategy;
use crate::{GameState, Position};
use rand::{seq::SliceRandom, thread_rng};
use std::fmt::{self, Display};

#[derive(Default)]
pub struct RandomMove;

impl Display for RandomMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(RandomMove))
    }
}

impl Strategy for RandomMove {
    fn decide(&self, gs: &GameState) -> Position {
        *gs.moves().choose(&mut thread_rng()).unwrap()
    }
}

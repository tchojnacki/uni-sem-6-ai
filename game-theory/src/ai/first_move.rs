use super::strategy::Strategy;
use crate::{GameState, Position};
use std::fmt::{self, Display};

#[derive(Default)]
pub struct FirstMove;

impl Display for FirstMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(FirstMove))
    }
}

impl Strategy for FirstMove {
    fn decide(&self, gs: &GameState) -> Position {
        *gs.moves().first().unwrap()
    }
}

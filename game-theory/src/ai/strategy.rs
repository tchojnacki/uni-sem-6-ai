use crate::{GameState, Position};
use std::fmt::Display;

pub trait Strategy: Display + Sync {
    fn decide(&self, gs: &GameState) -> Position;
}

pub trait TreeVisitingStrategy: Strategy {
    #[must_use]
    fn visited(&self) -> u32;
}

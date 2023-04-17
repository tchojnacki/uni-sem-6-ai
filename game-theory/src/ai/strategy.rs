use crate::{GameState, Position};
use std::fmt::Display;

pub trait Strategy: Display + Sync {
    fn decide(&self, gs: &GameState) -> Position;
}

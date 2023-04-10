use crate::{GameState, Position};
use std::fmt::Display;

pub trait Strategy: Display {
    fn decide(&mut self, gs: &GameState) -> Position;
}

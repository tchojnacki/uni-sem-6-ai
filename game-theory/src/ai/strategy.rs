use crate::{GameState, Position};

pub trait Strategy {
    fn decide(&mut self, gs: &GameState) -> Position;
}

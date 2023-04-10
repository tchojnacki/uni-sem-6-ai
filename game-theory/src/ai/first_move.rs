use super::strategy::Strategy;
use crate::{GameState, Position};

#[derive(Default)]
pub struct FirstMove;

impl Strategy for FirstMove {
    fn decide(&mut self, gs: &GameState) -> Position {
        let mut moves = gs.valid_moves().collect::<Vec<Position>>();
        moves.sort_by_key(|m| m.index());
        moves[0]
    }
}

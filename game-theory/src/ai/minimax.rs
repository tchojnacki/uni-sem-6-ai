use super::{
    heuristics::{Heuristic, MAX_PLAYER, MIN_PLAYER},
    Strategy,
};
use crate::{GameState, Position};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

pub struct Minimax {
    heuristic: Heuristic,
    max_depth: u32,
}

impl Minimax {
    pub const fn new(heuristic: Heuristic, max_depth: u32) -> Self {
        Self {
            heuristic,
            max_depth,
        }
    }

    #[must_use]
    fn minimax(&self, gs: &GameState, depth: u32) -> (f64, Option<Position>) {
        if let Some(outcome) = gs.outcome() {
            return (outcome.evaluate(), None);
        }

        if depth == 0 {
            return (self.heuristic.evaluate(gs), None);
        }

        let mut moves = gs.moves();
        let mut best_pos = moves.pop().unwrap();
        let (mut best_eval, _) = self.minimax(&gs.make_move(best_pos), depth - 1);
        for position in moves {
            let (eval, _) = self.minimax(&gs.make_move(position), depth - 1);
            if matches!(
                (gs.turn(), eval.partial_cmp(&best_eval).unwrap()),
                (MAX_PLAYER, Ordering::Greater) | (MIN_PLAYER, Ordering::Less),
            ) {
                best_eval = eval;
                best_pos = position;
            }
        }
        (best_eval, Some(best_pos))
    }
}

impl Display for Minimax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MM({}, {})", self.heuristic, self.max_depth)
    }
}

impl Strategy for Minimax {
    fn decide(&self, gs: &GameState) -> crate::Position {
        let (_, pos) = self.minimax(gs, self.max_depth);
        pos.unwrap()
    }
}

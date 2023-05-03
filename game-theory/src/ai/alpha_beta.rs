use super::{
    heuristics::{Heuristic, MAX_PLAYER, MIN_PLAYER},
    Strategy,
};
use crate::{GameState, Position};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
    sync::atomic::{self, AtomicU32},
};

#[must_use]
pub struct AlphaBeta {
    heuristic: Heuristic,
    max_depth: u32,
    visited: AtomicU32,
}

impl AlphaBeta {
    pub const fn new(heuristic: Heuristic, max_depth: u32) -> Self {
        Self {
            heuristic,
            max_depth,
            visited: AtomicU32::new(0),
        }
    }

    #[must_use]
    pub fn visited(&self) -> u32 {
        self.visited.load(atomic::Ordering::Relaxed)
    }

    #[must_use]
    fn alpha_beta(
        &self,
        gs: &GameState,
        depth: u32,
        mut alpha: f64,
        mut beta: f64,
    ) -> (f64, Option<Position>) {
        self.visited.fetch_add(1, atomic::Ordering::Relaxed);

        if let Some(outcome) = gs.outcome() {
            return (outcome.evaluate(), None);
        }

        if depth == 0 {
            return (self.heuristic.evaluate(gs), None);
        }

        let mut moves = gs.moves();
        let mut best_pos = moves.pop().unwrap();
        let (mut best_eval, _) = self.alpha_beta(&gs.make_move(best_pos), depth - 1, alpha, beta);
        for position in moves {
            let (eval, _) = self.alpha_beta(&gs.make_move(position), depth - 1, alpha, beta);
            if matches!(
                (gs.turn(), eval.partial_cmp(&best_eval).unwrap()),
                (MAX_PLAYER, Ordering::Greater) | (MIN_PLAYER, Ordering::Less),
            ) {
                best_eval = eval;
                best_pos = position;
            }

            // Alpha-beta pruning
            match gs.turn() {
                MAX_PLAYER => alpha = alpha.max(eval),
                MIN_PLAYER => beta = beta.min(eval),
            }
            if beta <= alpha {
                break;
            }
        }
        (best_eval, Some(best_pos))
    }
}

impl Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "αβ({}, {})", self.heuristic, self.max_depth)
    }
}

impl Strategy for AlphaBeta {
    fn decide(&self, gs: &GameState) -> crate::Position {
        let (_, pos) = self.alpha_beta(gs, self.max_depth, f64::NEG_INFINITY, f64::INFINITY);
        pos.unwrap()
    }
}

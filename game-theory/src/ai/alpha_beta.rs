use super::{
    minimax::{MAX_PLAYER, MIN_PLAYER},
    Strategy,
};
use crate::{GameState, Outcome, Position};
use std::fmt::{self, Display};

fn alpha_beta(
    gs: &GameState,
    depth: u32,
    mut alpha: f64,
    mut beta: f64,
) -> (f64, Option<Position>) {
    if let Some(outcome) = gs.outcome() {
        return (
            match outcome {
                Outcome::Winner(MAX_PLAYER) => f64::INFINITY,
                Outcome::Winner(MIN_PLAYER) => f64::NEG_INFINITY,
                Outcome::Draw => 0.,
            },
            None,
        );
    }

    if depth == 0 {
        // TODO: Heuristic
        return (
            gs.score_of(MAX_PLAYER) as f64 - gs.score_of(MIN_PLAYER) as f64,
            None,
        );
    }

    if gs.turn() == MAX_PLAYER {
        let (mut max_eval, mut max_pos) = (f64::NEG_INFINITY, None);
        for &position in gs.moves() {
            let (eval, _) = alpha_beta(&gs.make_move(position), depth - 1, alpha, beta);
            max_eval = max_eval.max(eval);
            max_pos = Some(position);
            alpha = alpha.max(eval);
            if beta <= alpha {
                break;
            }
        }
        (max_eval, max_pos)
    } else {
        let (mut min_eval, mut min_pos) = (f64::INFINITY, None);
        for &position in gs.moves() {
            let (eval, _) = alpha_beta(&gs.make_move(position), depth - 1, alpha, beta);
            min_eval = min_eval.min(eval);
            min_pos = Some(position);
            beta = beta.min(eval);
            if beta <= alpha {
                break;
            }
        }
        (min_eval, min_pos)
    }
}

pub struct AlphaBeta {
    max_depth: u32,
}

impl AlphaBeta {
    pub const fn new(max_depth: u32) -> Self {
        Self { max_depth }
    }
}

impl Display for AlphaBeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "αβ({})", self.max_depth)
    }
}

impl Strategy for AlphaBeta {
    fn decide(&mut self, gs: &GameState) -> crate::Position {
        let (_, pos) = alpha_beta(gs, self.max_depth, f64::NEG_INFINITY, f64::INFINITY);
        pos.unwrap()
    }
}

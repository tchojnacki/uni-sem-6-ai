use super::Strategy;
use crate::{GameState, Outcome, Player, Position};
use std::fmt::{self, Display};

const MAX_PLAYER: Player = Player::Black;
const MIN_PLAYER: Player = Player::White;

fn minimax(gs: &GameState, depth: u32) -> (f64, Option<Position>) {
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
        for position in gs.valid_moves() {
            let (eval, _) = minimax(&gs.make_move(position), depth - 1);
            max_eval = max_eval.max(eval);
            max_pos = Some(position);
        }
        (max_eval, max_pos)
    } else {
        let (mut min_eval, mut min_pos) = (f64::INFINITY, None);
        for position in gs.valid_moves() {
            let (eval, _) = minimax(&gs.make_move(position), depth - 1);
            min_eval = min_eval.min(eval);
            min_pos = Some(position);
        }
        (min_eval, min_pos)
    }
}

pub struct Minimax {
    max_depth: u32,
}

impl Minimax {
    pub const fn new(max_depth: u32) -> Self {
        Self { max_depth }
    }
}

impl Display for Minimax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MM({})", self.max_depth)
    }
}

impl Strategy for Minimax {
    fn decide(&mut self, gs: &GameState) -> crate::Position {
        let (_, pos) = minimax(gs, self.max_depth);
        pos.unwrap()
    }
}

use super::{heuristics::Heuristic, Strategy};
use crate::{GameState, Outcome, Player, Position};
use std::fmt::{self, Display};

pub const MAX_PLAYER: Player = Player::Black;
pub const MIN_PLAYER: Player = Player::White;

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
            return (self.heuristic.evaluate(gs), None);
        }

        if gs.turn() == MAX_PLAYER {
            let (mut max_eval, mut max_pos) = (f64::NEG_INFINITY, None);
            for position in gs.moves() {
                let (eval, _) = self.minimax(&gs.make_move(position), depth - 1);
                if eval >= max_eval {
                    max_eval = eval;
                    max_pos = Some(position);
                }
            }
            (max_eval, max_pos)
        } else {
            let (mut min_eval, mut min_pos) = (f64::INFINITY, None);
            for position in gs.moves() {
                let (eval, _) = self.minimax(&gs.make_move(position), depth - 1);
                if eval <= min_eval {
                    min_eval = eval;
                    min_pos = Some(position);
                }
            }
            (min_eval, min_pos)
        }
    }
}

impl Display for Minimax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MM({}, {})", self.heuristic, self.max_depth)
    }
}

impl Strategy for Minimax {
    fn decide(&mut self, gs: &GameState) -> crate::Position {
        let (_, pos) = self.minimax(gs, self.max_depth);
        pos.unwrap()
    }
}

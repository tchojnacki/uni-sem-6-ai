mod alpha_beta;
mod first_move;
mod greedy;
mod heuristics;
mod minimax;
mod player_input;
mod random_move;
mod strategy;

pub use alpha_beta::AlphaBeta;
pub use first_move::FirstMove;
pub use greedy::Greedy;
pub use heuristics::{Heuristic, WEIGHTS_KORMAN, WEIGHTS_MAGGS, WEIGHTS_SANNIDHANAM};
pub use minimax::Minimax;
pub use player_input::PlayerInput;
pub use random_move::RandomMove;
pub use strategy::Strategy;

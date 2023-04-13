mod alpha_beta;
mod corners_greedy;
mod first_move;
mod heuristics;
mod minimax;
mod player_input;
mod random_move;
mod score_greedy;
mod strategy;

pub use alpha_beta::AlphaBeta;
pub use corners_greedy::CornersGreedy;
pub use first_move::FirstMove;
pub use heuristics::{Heuristic, WEIGHTS_KORMAN, WEIGHTS_MAGGS, WEIGHTS_SANNIDHANAM};
pub use minimax::Minimax;
pub use player_input::PlayerInput;
pub use random_move::RandomMove;
pub use score_greedy::ScoreGreedy;
pub use strategy::Strategy;

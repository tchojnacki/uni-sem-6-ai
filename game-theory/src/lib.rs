pub mod ai;
pub mod bitboard;
pub mod cli;
pub mod elo;
mod game_state;
mod outcome;
mod player;
mod position;
mod square;
mod styles;
mod tournament;

pub use game_state::GameState;
pub use outcome::Outcome;
pub use player::Player;
pub use position::{Position, BOARD_SIDE, BOARD_SQUARES};
pub use styles::strip_string;
pub use tournament::run_tournament;

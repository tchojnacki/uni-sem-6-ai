mod game_state;
mod outcome;
mod player;
mod position;
mod square;
mod styles;

pub use game_state::GameState;
pub use outcome::Outcome;
pub use player::Player;
pub use position::{p, Position, BOARD_SIDE, BOARD_SQUARES};
pub use styles::strip_string;

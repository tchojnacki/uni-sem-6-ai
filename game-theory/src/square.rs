use crate::player::Player;
use colored::Colorize;
use std::fmt::{self, Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty,
    Placed(Player),
}

impl Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Square::Empty => "0".bright_green().on_green(),
                Square::Placed(Player::Black) => "1".bright_black().on_black(),
                Square::Placed(Player::White) => "2".bright_black().on_white(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn has_correct_mem_repr() {
        assert_eq!(mem::size_of::<Square>(), 1);
    }
}

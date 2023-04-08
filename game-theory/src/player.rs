use colored::Colorize;
use std::fmt::{self, Display};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    Black = 1,
    White = 2,
}

impl Player {
    pub const fn opponent(&self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::Black => "Black".bright_black(),
                Player::White => "White".bright_white(),
            }
            .bold()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn has_correct_mem_repr() {
        assert_eq!(mem::size_of::<Player>(), 1);
        assert_eq!(Player::Black as usize, 1);
        assert_eq!(Player::White as usize, 2);
    }

    #[test]
    fn opponent_returns_correct_player() {
        assert_eq!(Player::Black.opponent(), Player::White);
        assert_eq!(Player::White.opponent(), Player::Black);
    }
}

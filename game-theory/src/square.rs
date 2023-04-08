use crate::{
    player::Player,
    styles::{BLACK_BG, BLACK_FG, EMPTY_BG, EMPTY_FG, WHITE_BG, WHITE_FG},
};
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
                Square::Empty => "0".color(EMPTY_FG).on_color(EMPTY_BG),
                Square::Placed(Player::Black) => "1".color(BLACK_FG).on_color(BLACK_BG),
                Square::Placed(Player::White) => "2".color(WHITE_FG).on_color(WHITE_BG),
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

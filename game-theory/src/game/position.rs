use super::bitboard as bb;
use std::fmt::{self, Display};

pub const BOARD_SIDE: usize = 8;
pub const BOARD_SQUARES: usize = BOARD_SIDE.pow(2);

const COL_NOTATION: &str = "ABCDEFGH";
const ROW_NOTATION: &str = "12345678";

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position(u8);

#[cfg(test)]
pub fn p(notation: &'static str) -> Position {
    Position::from(notation).unwrap()
}

impl Position {
    #[must_use]
    pub fn from(notation: &str) -> Option<Self> {
        let mut chars = notation.chars();
        let col_char = chars.next()?;
        let row_char = chars.next()?;
        if chars.next().is_some() {
            return None;
        }
        let col = COL_NOTATION.chars().position(|c| c == col_char)?;
        let row = ROW_NOTATION.chars().position(|c| c == row_char)?;
        Some(Position((row * BOARD_SIDE + col) as u8))
    }

    pub fn from_index(index: usize) -> Position {
        if index >= BOARD_SQUARES {
            panic!("Invalid position index!")
        }

        Position(index as u8)
    }

    #[must_use]
    pub const fn index(&self) -> usize {
        self.0 as usize
    }

    pub fn neighbours(&self) -> impl Iterator<Item = Self> {
        bb::positions(bb::neighbours(bb::from_pos(*self))).into_iter()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            COL_NOTATION.chars().nth(self.index() % 8).unwrap(),
            ROW_NOTATION.chars().nth(self.index() / 8).unwrap()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Position {
        fn arbitrary(gen: &mut quickcheck::Gen) -> Self {
            *gen.choose(&(0..=63).map(Position::from_index).collect::<Vec<_>>())
                .unwrap()
        }
    }

    #[test]
    fn from_works_for_valid_notations() {
        // Corners
        assert_eq!(Position::from("A1"), Some(Position(0)));
        assert_eq!(Position::from("H1"), Some(Position(7)));
        assert_eq!(Position::from("A8"), Some(Position(56)));
        assert_eq!(Position::from("H8"), Some(Position(63)));
        // Center squares
        assert_eq!(Position::from("D4"), Some(Position(27)));
        assert_eq!(Position::from("E4"), Some(Position(28)));
        assert_eq!(Position::from("D5"), Some(Position(35)));
        assert_eq!(Position::from("E5"), Some(Position(36)));
        // Others
        assert_eq!(Position::from("C2"), Some(Position(10)));
        assert_eq!(Position::from("D6"), Some(Position(43)));
        assert_eq!(Position::from("G1"), Some(Position(6)));
        assert_eq!(Position::from("A5"), Some(Position(32)));
    }

    #[test]
    fn from_returns_none_for_notation_mistakes() {
        // Shorter strings
        assert_eq!(Position::from(""), None);
        assert_eq!(Position::from("B"), None);
        assert_eq!(Position::from("5"), None);
        // Letters out of range
        assert_eq!(Position::from("I1"), None);
        assert_eq!(Position::from("?3"), None);
        assert_eq!(Position::from("Z6"), None);
        // Numbers out of range
        assert_eq!(Position::from("C0"), None);
        assert_eq!(Position::from("A9"), None);
        assert_eq!(Position::from("HD"), None);
        // Longer strings
        assert_eq!(Position::from("A12"), None);
        assert_eq!(Position::from("  D5"), None);
        assert_eq!(Position::from("C3\n"), None);
    }

    #[quickcheck]
    fn to_string_and_from_are_inverses(position: Position) -> bool {
        Position::from(&position.to_string()) == Some(position)
    }

    #[quickcheck]
    fn index_and_from_index_are_inverses(position: Position) -> bool {
        Position::from_index(position.index()) == position
    }
}

use std::fmt::{self, Display};

pub const BOARD_SIDE: usize = 8;
pub const BOARD_SQUARES: usize = BOARD_SIDE.pow(2);

const COL_NOTATION: &str = "ABCDEFGH";
const ROW_NOTATION: &str = "12345678";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position(u8);

pub fn p(notation: &'static str) -> Position {
    Position::from(notation).unwrap()
}

impl Position {
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

    pub fn all() -> impl Iterator<Item = Self> {
        (0..BOARD_SQUARES as u8).map(Position)
    }

    pub fn corners() -> impl Iterator<Item = Self> {
        [p("A1"), p("H1"), p("A8"), p("H8")].into_iter()
    }

    pub const CENTER_SQUARES: [Position; 4] = [
        Position(27), // D4
        Position(28), // E4
        Position(35), // D5
        Position(36), // E5
    ];

    pub const fn index(&self) -> usize {
        self.0 as usize
    }

    pub const fn offset(&self, by: (i32, i32)) -> Option<Self> {
        let col = (self.index() % BOARD_SIDE) as i32 + by.0;
        let row = (self.index() / BOARD_SIDE) as i32 + by.1;

        if 0 <= col && col < BOARD_SIDE as i32 && 0 <= row && row < BOARD_SIDE as i32 {
            Some(Position((row * BOARD_SIDE as i32 + col) as u8))
        } else {
            None
        }
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

    #[test]
    fn to_string_and_from_are_inverses() {
        fn test(before: Position) {
            let after = Position::from(&before.to_string());
            assert_eq!(Some(before), after);
        }

        test(p("A1"));
        test(p("C3"));
        test(p("H7"));
        test(p("D4"));
    }

    #[test]
    fn all_returns_64_squares() {
        assert_eq!(Position::all().count(), BOARD_SQUARES);
    }

    #[test]
    fn offset_returns_moved_pos_for_valid_args() {
        // Zero offset
        assert_eq!(p("A3").offset((0, 0)), Some(p("A3")));
        assert_eq!(p("E8").offset((0, 0)), Some(p("E8")));
        // Column offset
        assert_eq!(p("B3").offset((3, 0)), Some(p("E3")));
        assert_eq!(p("B1").offset((-1, 0)), Some(p("A1")));
        // Row offset
        assert_eq!(p("C7").offset((0, 1)), Some(p("C8")));
        assert_eq!(p("A5").offset((0, -2)), Some(p("A3")));
        // Mixed offset
        assert_eq!(p("C4").offset((3, 2)), Some(p("F6")));
        assert_eq!(p("B2").offset((-1, 6)), Some(p("A8")));
    }

    #[test]
    fn offset_returns_none_for_invalid_args() {
        // Top-left corner
        assert_eq!(p("A1").offset((-1, 0)), None);
        assert_eq!(p("A1").offset((0, -1)), None);
        // Top-right corner
        assert_eq!(p("H1").offset((1, 0)), None);
        assert_eq!(p("H1").offset((0, -1)), None);
        // Bottom-left corner
        assert_eq!(p("A8").offset((-1, 0)), None);
        assert_eq!(p("A8").offset((0, 1)), None);
        // Bottom-right corner
        assert_eq!(p("H8").offset((1, 0)), None);
        assert_eq!(p("H8").offset((0, 1)), None);
        // Top side
        assert_eq!(p("G1").offset((1, -2)), None);
        assert_eq!(p("D1").offset((-2, -1)), None);
        // Right side
        assert_eq!(p("H4").offset((3, -2)), None);
        assert_eq!(p("H1").offset((1, 3)), None);
        // Bottom side
        assert_eq!(p("C8").offset((3, 1)), None);
        assert_eq!(p("H8").offset((-4, 3)), None);
        // Left side
        assert_eq!(p("A2").offset((-3, 0)), None);
        assert_eq!(p("A7").offset((-2, 1)), None);
        // From center
        assert_eq!(p("D4").offset((5, 0)), None);
        assert_eq!(p("E5").offset((-5, -5)), None);
    }
}

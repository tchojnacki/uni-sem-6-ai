const BOARD_SIDE: usize = 8;
pub const BOARD_SQUARES: usize = BOARD_SIDE.pow(2);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position(usize);

impl Position {
    pub fn all() -> impl Iterator<Item = Self> {
        (0..BOARD_SQUARES).map(Position)
    }

    pub const CENTER_SQUARES: [Position; 4] = [
        Position(27), // D4
        Position(28), // E4
        Position(35), // D5
        Position(36), // E5
    ];

    pub const fn index(&self) -> usize {
        self.0
    }

    pub const fn offset(&self, by: (i32, i32)) -> Option<Self> {
        let col = (self.0 % BOARD_SIDE) as i32 + by.0;
        let row = (self.0 / BOARD_SIDE) as i32 + by.1;

        if 0 <= col && col < BOARD_SIDE as i32 && 0 <= row && row < BOARD_SIDE as i32 {
            Some(Position((row * BOARD_SIDE as i32 + col) as usize))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_returns_64_squares() {
        assert_eq!(Position::all().count(), BOARD_SQUARES);
    }

    #[test]
    fn offset_returns_moved_pos_for_valid_args() {
        // Zero offset
        assert_eq!(Position(16).offset((0, 0)), Some(Position(16)));
        assert_eq!(Position(60).offset((0, 0)), Some(Position(60)));
        // Column offset
        assert_eq!(Position(17).offset((3, 0)), Some(Position(20)));
        assert_eq!(Position(1).offset((-1, 0)), Some(Position(0)));
        // Row offset
        assert_eq!(Position(50).offset((0, 1)), Some(Position(58)));
        assert_eq!(Position(32).offset((0, -2)), Some(Position(16)));
        // Mixed offset
        assert_eq!(Position(26).offset((3, 2)), Some(Position(45)));
        assert_eq!(Position(9).offset((-1, 6)), Some(Position(56)));
    }

    #[test]
    fn offset_returns_none_for_invalid_args() {
        // Top-left corner
        assert_eq!(Position(0).offset((-1, 0)), None);
        assert_eq!(Position(0).offset((0, -1)), None);
        // Top-right corner
        assert_eq!(Position(7).offset((1, 0)), None);
        assert_eq!(Position(7).offset((0, -1)), None);
        // Bottom-left corner
        assert_eq!(Position(56).offset((-1, 0)), None);
        assert_eq!(Position(56).offset((0, 1)), None);
        // Bottom-right corner
        assert_eq!(Position(63).offset((1, 0)), None);
        assert_eq!(Position(63).offset((0, 1)), None);
        // Top side
        assert_eq!(Position(6).offset((1, -2)), None);
        assert_eq!(Position(3).offset((-2, -1)), None);
        // Right side
        assert_eq!(Position(31).offset((3, -2)), None);
        assert_eq!(Position(7).offset((1, 3)), None);
        // Bottom side
        assert_eq!(Position(58).offset((3, 1)), None);
        assert_eq!(Position(63).offset((-4, 3)), None);
        // Left side
        assert_eq!(Position(8).offset((-3, 0)), None);
        assert_eq!(Position(48).offset((-2, 1)), None);
        // From center
        assert_eq!(Position(27).offset((5, 0)), None);
        assert_eq!(Position(36).offset((-5, -5)), None);
    }
}

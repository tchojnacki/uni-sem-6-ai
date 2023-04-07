use std::ops::Range;

const BOARD_SIDE: usize = 8;
const SIDE_RANGE: Range<i32> = 0..BOARD_SIDE as i32;
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

    pub fn index(&self) -> usize {
        self.0
    }

    pub fn offset(&self, by: (i32, i32)) -> Option<Self> {
        let col = (self.0 % BOARD_SIDE) as i32 + by.0;
        let row = (self.0 / BOARD_SIDE) as i32 + by.1;

        if SIDE_RANGE.contains(&col) && SIDE_RANGE.contains(&row) {
            Some(Position((row * BOARD_SIDE as i32 + col) as usize))
        } else {
            None
        }
    }
}

use std::{collections::HashSet, ops::Range};

const BOARD_SIDE: usize = 8;
const SIDE_RANGE: Range<i32> = 0..BOARD_SIDE as i32;
const BOARD_SQUARES: usize = BOARD_SIDE.pow(2);

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Black = 1,
    White = 2,
}

impl Player {
    pub fn opposite(&self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Empty,
    Placed(Player),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position(usize);

impl Position {
    pub const DIRECTIONS: [(i32, i32); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    pub fn all() -> impl Iterator<Item = Self> {
        (0..BOARD_SQUARES).map(Position)
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

pub struct GameState {
    turn: Player,
    board: [Square; BOARD_SQUARES],
}

impl GameState {
    pub fn reversi_initial() -> Self {
        Self {
            turn: Player::Black,
            board: [Square::Empty; BOARD_SQUARES],
        }
    }

    pub fn othello_initial() -> Self {
        let mut board = [Square::Empty; BOARD_SQUARES];
        board[27] = Square::Placed(Player::White); // D4
        board[28] = Square::Placed(Player::Black); // E4
        board[35] = Square::Placed(Player::Black); // D5
        board[36] = Square::Placed(Player::White); // E5
        Self {
            turn: Player::Black,
            board,
        }
    }

    fn at(&self, pos: Position) -> Square {
        self.board[pos.0]
    }

    pub fn valid_moves(&self) -> impl Iterator<Item = Position> + '_ {
        let mut result = HashSet::new();

        if Position::all()
            .filter(|&pos| matches!(self.at(pos), Square::Placed(_)))
            .count()
            < 4
        {
            result.extend(
                [27, 28, 35, 36]
                    .map(Position)
                    .into_iter()
                    .filter(|&p| self.at(p) == Square::Empty),
            );
            return result.into_iter();
        }

        for pos in Position::all().filter(|&pos| self.at(pos) == Square::Placed(self.turn)) {
            for dir in Position::DIRECTIONS {
                if let Some(mut coord) = pos.offset(dir) {
                    while self.at(coord) == Square::Placed(self.turn.opposite()) {
                        if let Some(next) = coord.offset(dir) {
                            coord = next;
                            if self.at(coord) == Square::Empty {
                                result.insert(coord);
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        result.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn player_has_correct_repr() {
        assert_eq!(mem::size_of::<Player>(), 1);
        assert_eq!(Player::Black as usize, 1);
        assert_eq!(Player::White as usize, 2);
    }

    #[test]
    fn square_has_correct_repr() {
        assert_eq!(mem::size_of::<Square>(), 1);
    }
}

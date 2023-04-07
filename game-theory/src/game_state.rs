use crate::{
    player::Player,
    position::{Position, BOARD_SQUARES},
    square::Square,
};
use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

pub struct GameState {
    turn: Player,
    board: [Square; BOARD_SQUARES],
}

impl GameState {
    pub const fn reversi_initial() -> Self {
        Self {
            turn: Player::Black,
            board: [Square::Empty; BOARD_SQUARES],
        }
    }

    pub const fn othello_initial() -> Self {
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

    const fn at(&self, pos: Position) -> Square {
        self.board[pos.index()]
    }

    fn player_discs(&self) -> impl Iterator<Item = Position> + '_ {
        Position::all().filter(|&pos| self.at(pos) == Square::Placed(self.turn))
    }

    fn opponent_discs(&self) -> impl Iterator<Item = Position> + '_ {
        Position::all().filter(|&pos| self.at(pos) == Square::Placed(self.turn.opponent()))
    }

    fn occupied_squares(&self) -> impl Iterator<Item = Position> + '_ {
        Position::all().filter(|&pos| matches!(self.at(pos), Square::Placed(_)))
    }

    fn empty_squares(&self) -> impl Iterator<Item = Position> + '_ {
        Position::all().filter(|&pos| self.at(pos) == Square::Empty)
    }

    pub fn valid_moves(&self) -> impl Iterator<Item = Position> + '_ {
        let mut result = HashSet::new();

        // Reversi earlygame variant
        if self.occupied_squares().count() < 4 {
            result.extend(
                Position::CENTER_SQUARES
                    .into_iter()
                    .filter(|&p| self.at(p) == Square::Empty),
            );
            return result.into_iter();
        }

        for pos in self.player_discs() {
            for dir in DIRECTIONS {
                if let Some(mut coord) = pos.offset(dir) {
                    while self.at(coord) == Square::Placed(self.turn.opponent()) {
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

    #[test]
    fn reversi_earlygame_is_correct() {
        let gs = GameState::reversi_initial();
        assert_eq!(gs.occupied_squares().count(), 0);

        let mut moves = gs.valid_moves().map(|p| p.index()).collect::<Vec<_>>();
        moves.sort();
        assert_eq!(moves, [27, 28, 35, 36]);
    }

    #[test]
    fn othello_earlygame_is_correct() {
        let gs = GameState::othello_initial();
        assert_eq!(gs.player_discs().count(), 2);
        assert_eq!(gs.opponent_discs().count(), 2);

        let mut moves = gs.valid_moves().map(|p| p.index()).collect::<Vec<_>>();
        moves.sort();
        assert_eq!(moves, [19, 26, 37, 44]);
    }
}

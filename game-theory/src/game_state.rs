use crate::{
    player::Player,
    position::{p, Position, BOARD_SIDE, BOARD_SQUARES},
    square::Square,
    styles::{strip_string, EMPTY_BG, VALID_FG},
};
use colored::Colorize;
use std::{
    collections::HashSet,
    fmt::{self, Display},
};

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

    const fn at(&self, position: Position) -> Square {
        self.board[position.index()]
    }

    fn discs_of(&self, player: Player) -> impl Iterator<Item = Position> + '_ {
        Position::all().filter(move |&pos| self.at(pos) == Square::Placed(player))
    }

    fn occupied_squares(&self) -> impl Iterator<Item = Position> + '_ {
        Position::all().filter(|&pos| matches!(self.at(pos), Square::Placed(_)))
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

        for position in self.discs_of(self.turn) {
            for dir in DIRECTIONS {
                if let Some(mut coord) = position.offset(dir) {
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

    fn is_valid(&self, position: Position) -> bool {
        // TODO: Find faster way to check validity
        self.valid_moves().any(|p| p == position)
    }

    pub fn make_move(&self, position: Position) -> Option<GameState> {
        if !self.is_valid(position) {
            return None;
        }

        let mut next_board = self.board;
        next_board[position.index()] = Square::Placed(self.turn);

        for dir in DIRECTIONS {
            let mut current = position;
            let mut flip_queue = Vec::new();
            while let Some(next) = current.offset(dir) {
                current = next;
                match self.at(current) {
                    Square::Placed(p) if p == self.turn.opponent() => flip_queue.push(current),
                    Square::Placed(_) => {
                        flip_queue
                            .iter()
                            .for_each(|p| next_board[p.index()] = Square::Placed(self.turn));
                        break;
                    }
                    Square::Empty => break,
                }
            }
        }

        // TODO: Endgame logic (winning game, passing rounds etc.)

        Some(GameState {
            turn: self.turn.opponent(),
            board: next_board,
        })
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..BOARD_SIDE as i32 {
            for col in 0..BOARD_SIDE as i32 {
                let position = p("A1").offset((col, row)).unwrap();
                let mut square_str = self.at(position).to_string();
                if self.is_valid(position) {
                    square_str = strip_string(&square_str)
                        .color(VALID_FG)
                        .on_color(EMPTY_BG)
                        .to_string();
                }
                write!(f, "{}", square_str)?;
            }
            writeln!(f)?;
        }

        let black = self.discs_of(Player::Black).count();
        let white = self.discs_of(Player::White).count();

        writeln!(
            f,
            "Turn: {} | Score: {}-{} | Winner: {}",
            self.turn,
            black.to_string().bright_black(),
            white.to_string().bright_white(),
            "-" // TODO
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::p;

    fn assert_valid_moves(gs: &GameState, expected: &[Position]) {
        let mut moves = gs.valid_moves().collect::<Vec<_>>();
        moves.sort_by_key(|p| p.index());
        assert_eq!(moves, expected);
    }

    // TODO: is_valid tests

    #[test]
    fn reversi_earlygame() {
        let gs = GameState::reversi_initial();
        assert_eq!(gs.occupied_squares().count(), 0);
        assert_valid_moves(&gs, &[p("D4"), p("E4"), p("D5"), p("E5")]);

        let gs = gs.make_move(p("D5")).unwrap();
        assert_valid_moves(&gs, &[p("D4"), p("E4"), p("E5")]);

        let gs = gs.make_move(p("E4")).unwrap();
        assert_valid_moves(&gs, &[p("D4"), p("E5")]);

        let gs = gs.make_move(p("D4")).unwrap();
        assert_valid_moves(&gs, &[p("E5")]);

        let gs = gs.make_move(p("E5")).unwrap();

        // No flipping in first four moves
        assert_eq!(gs.at(p("D5")), Square::Placed(Player::Black));
        assert_eq!(gs.at(p("E4")), Square::Placed(Player::White));
        assert_eq!(gs.at(p("D4")), Square::Placed(Player::Black));
        assert_eq!(gs.at(p("E5")), Square::Placed(Player::White));
    }

    #[test]
    fn othello_earlygame() {
        let gs = GameState::othello_initial();
        assert_eq!(gs.discs_of(gs.turn).count(), 2);
        assert_eq!(gs.discs_of(gs.turn.opponent()).count(), 2);
        assert_valid_moves(&gs, &[p("D3"), p("C4"), p("F5"), p("E6")]);

        // From: https://www.eothello.com/game-rules

        let gs = gs.make_move(p("D3")).unwrap();
        assert_valid_moves(&gs, &[p("C3"), p("E3"), p("C5")]);

        let gs = gs.make_move(p("C5")).unwrap();
        assert_valid_moves(&gs, &[p("B6"), p("C6"), p("D6"), p("E6"), p("F6")]);
    }
}

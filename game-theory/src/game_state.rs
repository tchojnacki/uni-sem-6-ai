use crate::{
    ai::{RandomMove, Strategy},
    bitboard::{
        diagonals, get_moves, has, make_move, positions, square, Bitboard, EMPTY,
        OTHELLO_BLACK_START, OTHELLO_WHITE_START,
    },
    player::Player,
    position::{Position, BOARD_SIDE, BOARD_SQUARES},
    square::Square,
    styles::{strip_string, EMPTY_BG, VALID_FG},
    Outcome,
};
use colored::Colorize;
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{self, Display},
    hash::Hash,
    time::{Duration, Instant},
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    turn: Player,
    black: Bitboard,
    white: Bitboard,
}

impl GameState {
    pub const fn turn(&self) -> Player {
        self.turn
    }

    const fn bitboard(&self, player: Player) -> Bitboard {
        match player {
            Player::Black => self.black,
            Player::White => self.white,
        }
    }

    pub const fn count_of(&self, player: Player) -> usize {
        self.bitboard(player).count_ones() as usize
    }

    pub const fn reversi_initial() -> Self {
        Self {
            turn: Player::Black,
            black: EMPTY,
            white: EMPTY,
        }
    }

    pub const fn othello_initial() -> Self {
        Self {
            turn: Player::Black,
            black: OTHELLO_BLACK_START,
            white: OTHELLO_WHITE_START,
        }
    }

    pub fn random_state_after(n: u32) -> Self {
        // TODO: panic
        let mut strategy = RandomMove::default();
        'outer: loop {
            let mut gs = Self::othello_initial();
            for _ in 0..n {
                if gs.move_bitboard() == EMPTY {
                    continue 'outer;
                }
                gs = gs.make_move(strategy.decide(&gs));
            }
            return gs;
        }
    }

    pub const fn at(&self, position: Position) -> Square {
        match (has(self.black, position), has(self.white, position)) {
            (false, false) => Square::Empty,
            (true, false) => Square::Placed(Player::Black),
            (false, true) => Square::Placed(Player::White),
            _ => unreachable!(),
        }
    }

    pub fn score_of(&self, player: Player) -> usize {
        match self.outcome() {
            Some(Outcome::Winner(p)) if p == player => {
                // counting empty squares for the winner
                BOARD_SQUARES - self.count_of(player.opponent())
            }
            Some(Outcome::Draw) => BOARD_SQUARES / 2,
            Some(Outcome::Winner(_)) | None => self.count_of(player),
        }
    }

    pub const fn move_bitboard(&self) -> Bitboard {
        get_moves(
            self.bitboard(self.turn),
            self.bitboard(self.turn.opponent()),
        )
    }

    pub fn moves(&self) -> Vec<Position> {
        positions(self.move_bitboard())
    }

    fn pass_if_required(&mut self) {
        if self.move_bitboard() == EMPTY {
            // No moves for opponent, pass
            self.turn = self.turn.opponent();
            if self.move_bitboard() == EMPTY {
                // No moves again, game is over, correct the player
                self.turn = self.turn.opponent();
            }
        }
    }

    pub fn make_move(&self, position: Position) -> GameState {
        let mut next_state = (*self).clone();
        match self.turn {
            Player::Black => make_move(position, &mut next_state.black, &mut next_state.white),
            Player::White => make_move(position, &mut next_state.white, &mut next_state.black),
        };

        next_state.turn = next_state.turn.opponent();
        next_state.pass_if_required();
        next_state
    }

    pub fn outcome(&self) -> Option<Outcome> {
        if self.move_bitboard() != EMPTY {
            return None;
        }

        Some(
            match self
                .count_of(Player::Black)
                .cmp(&self.count_of(Player::White))
            {
                Ordering::Less => Outcome::Winner(Player::White),
                Ordering::Equal => Outcome::Draw,
                Ordering::Greater => Outcome::Winner(Player::Black),
            },
        )
    }

    pub fn from_board_string_unverified(board_str: &str) -> Option<GameState> {
        let board_str = strip_string(board_str);
        if board_str.len() != BOARD_SQUARES {
            return None;
        }

        let mut black = EMPTY;
        let mut white = EMPTY;
        board_str.chars().enumerate().for_each(|(i, c)| match c {
            '0' => (),
            '1' => black |= 1 << i,
            '2' => white |= 1 << i,
            _ => unreachable!(), // strip_string should only leave 0, 1 and 2
        });

        let turn = if (black | white).count_ones() % 2 == 0 {
            Player::Black
        } else {
            Player::White
        };
        let mut result = GameState { turn, black, white };
        result.pass_if_required();
        Some(result)
    }

    fn original_discs(&self) -> (Bitboard, Bitboard) {
        let mut black = EMPTY;
        let mut white = EMPTY;
        for position in positions(self.black | self.white) {
            if !diagonals(position)
                .into_iter()
                .any(|diagonal| (diagonal & (self.black | self.white)).count_ones() == 2)
            {
                black |= self.black & square(position);
                white |= self.white & square(position);
            }
        }
        (black, white)
    }

    pub fn verify_reachability(&self, timeout: Duration) -> Option<bool> {
        let start_time = Instant::now();

        let target_bitboard = self.black | self.white;
        let (og_black, og_white) = self.original_discs();

        let mut stack = Vec::from([GameState::reversi_initial()]);
        let mut visited = HashSet::new();
        while let Some(current) = stack.pop() {
            if current == *self {
                return Some(true);
            }

            if Instant::now() - start_time >= timeout {
                return None;
            }

            for position in current.moves() {
                if !has(target_bitboard, position) {
                    continue;
                }
                match (
                    has(og_black, position),
                    has(og_white, position),
                    current.turn,
                ) {
                    (false, true, Player::Black) | (true, false, Player::White) => continue,
                    (true, true, _) => unreachable!(),
                    _ => (),
                }
                let next = current.make_move(position);
                if visited.contains(&next) {
                    continue;
                }
                visited.insert(next.clone());
                stack.push(next);
            }
        }

        Some(false)
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let moves = self.move_bitboard();
        for row in 0..BOARD_SIDE {
            for col in 0..BOARD_SIDE {
                let position = Position::from_index(row * BOARD_SIDE + col);
                let mut square_str = self.at(position).to_string();
                if has(moves, position) {
                    square_str = strip_string(&square_str)
                        .color(VALID_FG)
                        .on_color(EMPTY_BG)
                        .to_string();
                }
                write!(f, "{}", square_str)?;
            }
            writeln!(f)?;
        }

        writeln!(
            f,
            "Turn: {} | Score: {}-{} | Winner: {}",
            self.turn,
            self.score_of(Player::Black).to_string().bright_black(),
            self.score_of(Player::White).to_string().bright_white(),
            self.outcome()
                .map(|o| o.to_string())
                .unwrap_or(String::from("-"))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::p;

    fn assert_moves(gs: &GameState, expected: &[Position]) {
        let mut moves = gs.moves();
        moves.sort_by_key(|p| p.index());
        assert_eq!(moves, expected);
    }

    #[test]
    fn reversi_earlygame() {
        let gs = GameState::reversi_initial();
        assert_eq!(gs.black | gs.white, EMPTY);
        assert_moves(&gs, &[p("D4"), p("E4"), p("D5"), p("E5")]);

        let gs = gs.make_move(p("D5"));
        assert_moves(&gs, &[p("D4"), p("E4"), p("E5")]);

        let gs = gs.make_move(p("E4"));
        assert_moves(&gs, &[p("D4"), p("E5")]);

        let gs = gs.make_move(p("D4"));
        assert_moves(&gs, &[p("E5")]);

        let gs = gs.make_move(p("E5"));
        // No flipping in first four moves
        assert_eq!(gs.at(p("D5")), Square::Placed(Player::Black));
        assert_eq!(gs.at(p("E4")), Square::Placed(Player::White));
        assert_eq!(gs.at(p("D4")), Square::Placed(Player::Black));
        assert_eq!(gs.at(p("E5")), Square::Placed(Player::White));
    }

    #[test]
    fn othello_earlygame() {
        let gs = GameState::othello_initial();
        assert_eq!(gs.score_of(gs.turn), 2);
        assert_eq!(gs.score_of(gs.turn.opponent()), 2);
        assert_moves(&gs, &[p("D3"), p("C4"), p("F5"), p("E6")]);

        // From: https://www.eothello.com/game-rules
        let gs = gs.make_move(p("D3"));
        assert_moves(&gs, &[p("C3"), p("E3"), p("C5")]);

        let gs = gs.make_move(p("C5"));
        assert_moves(&gs, &[p("B6"), p("C6"), p("D6"), p("E6"), p("F6")]);
    }

    #[test]
    #[should_panic]
    fn invalid_moves_panic() {
        let gs = GameState::othello_initial();
        gs.make_move(p("A1"));
    }
}

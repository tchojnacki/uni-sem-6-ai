use crate::{
    ai::{RandomMove, Strategy},
    bitboard::{self as bb, Bitboard},
    player::Player,
    position::{Position, BOARD_SIDE, BOARD_SQUARES},
    square::Square,
    styles::{strip_string, EMPTY_BG, VALID_FG},
    Outcome,
};
use colored::Colorize;
use rand::{thread_rng, Rng};
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{self, Display},
    hash::Hash,
    time::{Duration, Instant},
};

#[must_use]
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct GameState {
    turn: Player,
    black: Bitboard,
    white: Bitboard,
}

impl GameState {
    pub const fn turn(&self) -> Player {
        self.turn
    }

    #[must_use]
    pub const fn bb_of(&self, player: Player) -> Bitboard {
        match player {
            Player::Black => self.black,
            Player::White => self.white,
        }
    }

    #[must_use]
    pub const fn occupied_bb(&self) -> Bitboard {
        self.black | self.white
    }

    #[must_use]
    pub const fn empty_bb(&self) -> Bitboard {
        !self.occupied_bb()
    }

    #[must_use]
    pub const fn score_of(&self, player: Player) -> u32 {
        self.bb_of(player).count_ones()
    }

    #[must_use]
    pub const fn move_number(&self) -> i32 {
        // -3 offsets number to be equal 1 for starting Othello board
        self.occupied_bb().count_ones() as i32 - 3
    }

    pub const fn reversi_initial() -> Self {
        Self {
            turn: Player::Black,
            black: bb::EMPTY,
            white: bb::EMPTY,
        }
    }

    pub const fn othello_initial() -> Self {
        Self {
            turn: Player::Black,
            black: bb::OTHELLO_BLACK_START,
            white: bb::OTHELLO_WHITE_START,
        }
    }

    pub fn random_state_between_inc(min_turn: i32, max_turn: i32) -> Self {
        assert!((-3..=60).contains(&min_turn));
        assert!((min_turn..=60).contains(&max_turn));

        // Move from Othello move space to Reversi round space
        let n = thread_rng().gen_range(min_turn..=max_turn) + 3;
        let strategy = RandomMove::default();
        let mut gs = Self::reversi_initial();
        for _ in 0..n {
            if gs.move_bb() == bb::EMPTY {
                return Self::random_state_between_inc(min_turn, max_turn);
            }
            gs = gs.make_move(strategy.decide(&gs));
        }
        gs
    }

    pub const fn at(&self, position: Position) -> Square {
        match (bb::has(self.black, position), bb::has(self.white, position)) {
            (false, false) => Square::Empty,
            (true, false) => Square::Placed(Player::Black),
            (false, true) => Square::Placed(Player::White),
            _ => unreachable!(),
        }
    }

    #[must_use]
    pub const fn move_bb(&self) -> Bitboard {
        bb::valid_moves(self.bb_of(self.turn), self.bb_of(self.turn.opponent()))
    }

    #[must_use]
    pub fn moves(&self) -> Vec<Position> {
        bb::positions(self.move_bb())
    }

    fn pass_if_required(&mut self) {
        if self.move_bb() == bb::EMPTY {
            // No moves for opponent, pass
            self.turn = self.turn.opponent();
            if self.move_bb() == bb::EMPTY {
                // No moves again, game is over, correct the player
                self.turn = self.turn.opponent();
            }
        }
    }

    pub fn make_move(&self, position: Position) -> Self {
        let mut next_state = (*self).clone();
        match self.turn {
            Player::Black => bb::make_move(position, &mut next_state.black, &mut next_state.white),
            Player::White => bb::make_move(position, &mut next_state.white, &mut next_state.black),
        };

        next_state.turn = next_state.turn.opponent();
        next_state.pass_if_required();

        assert_eq!(next_state.black & next_state.white, bb::EMPTY);

        next_state
    }

    #[must_use]
    pub fn outcome(&self) -> Option<Outcome> {
        if self.move_bb() != bb::EMPTY {
            return None;
        }

        Some(
            match self
                .score_of(Player::Black)
                .cmp(&self.score_of(Player::White))
            {
                Ordering::Less => Outcome::Winner(Player::White),
                Ordering::Equal => Outcome::Draw,
                Ordering::Greater => Outcome::Winner(Player::Black),
            },
        )
    }

    #[must_use]
    pub fn from_board_str_unverified(board_str: &str) -> Option<Self> {
        let board_str = strip_string(board_str);
        if board_str.len() != BOARD_SQUARES {
            return None;
        }

        let mut black = bb::EMPTY;
        let mut white = bb::EMPTY;
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
        assert_eq!(result.black & result.white, bb::EMPTY);
        Some(result)
    }

    #[must_use]
    fn original_discs(&self) -> (Bitboard, Bitboard) {
        let mut black = bb::EMPTY;
        let mut white = bb::EMPTY;
        for pos in bb::positions(self.occupied_bb()) {
            if !bb::diagonals(pos)
                .into_iter()
                .any(|diagonal| (diagonal & self.occupied_bb()).count_ones() == 2)
            {
                black |= self.black & bb::from_pos(pos);
                white |= self.white & bb::from_pos(pos);
            }
        }
        (black, white)
    }

    #[must_use]
    pub fn verify_reachability(&self, timeout: Duration) -> Option<bool> {
        let start_time = Instant::now();

        let target_bitboard = self.occupied_bb();
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

            for pos in current.moves() {
                if !bb::has(target_bitboard, pos) {
                    continue;
                }
                match (bb::has(og_black, pos), bb::has(og_white, pos), current.turn) {
                    (false, true, Player::Black) | (true, false, Player::White) => continue,
                    (true, true, _) => unreachable!(),
                    _ => (),
                }
                let next = current.make_move(pos);
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
        let moves = self.move_bb();
        for row in 0..BOARD_SIDE {
            for col in 0..BOARD_SIDE {
                let position = Position::from_index(row * BOARD_SIDE + col);
                let mut square_str = self.at(position).to_string();
                if bb::has(moves, position) {
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
            "Move number: {} | Turn: {} | Score: {}-{} | Winner: {}",
            self.move_number(),
            self.turn,
            self.score_of(Player::Black).to_string().bright_black(),
            self.score_of(Player::White).to_string().bright_white(),
            self.outcome()
                .map(|o| o.to_string())
                .unwrap_or(String::from("---"))
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::position::p;
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for GameState {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            Self::random_state_between_inc(1, 60)
        }
    }

    impl Arbitrary for Player {
        fn arbitrary(gen: &mut quickcheck::Gen) -> Self {
            *gen.choose(&[Player::Black, Player::White]).unwrap()
        }
    }

    fn assert_moves(gs: &GameState, expected: &[Position]) {
        let mut moves = gs.moves();
        moves.sort_by_key(|p| p.index());
        assert_eq!(moves, expected);
    }

    #[quickcheck]
    fn score_of_is_in_range(gs: GameState, player: Player) -> bool {
        (0..=64).contains(&gs.score_of(player))
    }

    #[test]
    fn move_number_is_consistent() {
        assert_eq!(GameState::othello_initial().move_number(), 1);
        assert_eq!(GameState::reversi_initial().move_number(), -3);
    }

    #[quickcheck]
    fn move_number_is_in_range(gs: GameState) -> bool {
        (1..=60).contains(&gs.move_number())
    }

    #[quickcheck]
    fn random_state_returns_correct_move_number() -> bool {
        let n = thread_rng().gen_range(1..=60);
        GameState::random_state_between_inc(n, n).move_number() == n
    }

    #[quickcheck]
    fn move_bb_has_same_move_count_as_moves(gs: GameState) -> bool {
        gs.move_bb().count_ones() == gs.moves().len() as u32
    }

    #[test]
    fn reversi_earlygame() {
        let gs = GameState::reversi_initial();
        assert_eq!(gs.occupied_bb(), bb::EMPTY);
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
        let _ = gs.make_move(p("A1"));
    }
}

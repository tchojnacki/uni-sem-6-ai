use crate::{
    ai::{RandomMove, Strategy},
    player::Player,
    position::{p, Position, BOARD_SIDE, BOARD_SQUARES},
    square::Square,
    styles::{strip_string, EMPTY_BG, VALID_FG},
    Outcome,
};
use colored::Colorize;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    hash::{Hash, Hasher},
    mem,
    time::{Duration, Instant},
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

fn precompute_moves(turn: Player, board: [Square; BOARD_SQUARES]) -> Box<[Position]> {
    let mut result = HashSet::new();

    // Reversi earlygame variant
    if board
        .iter()
        .filter(|s| matches!(s, Square::Placed(_)))
        .count()
        < 4
    {
        result.extend(
            Position::CENTER_SQUARES
                .into_iter()
                .filter(|&p| board[p.index()] == Square::Empty),
        );
    } else {
        for position in Position::all().filter(|p| board[p.index()] == Square::Placed(turn)) {
            for dir in DIRECTIONS {
                if let Some(mut coord) = position.offset(dir) {
                    while board[coord.index()] == Square::Placed(turn.opponent()) {
                        if let Some(next) = coord.offset(dir) {
                            coord = next;
                            if board[coord.index()] == Square::Empty {
                                result.insert(coord);
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }

    result.into_iter().collect()
}

#[derive(Clone)]
pub struct GameState {
    turn: Player,
    board: [Square; BOARD_SQUARES],
    moves: Box<[Position]>,
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.turn == other.turn && self.board == other.board
    }
}

impl Eq for GameState {}

impl Hash for GameState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.turn.hash(state);
        self.board.hash(state);
    }
}

impl GameState {
    pub const fn turn(&self) -> Player {
        self.turn
    }

    pub fn reversi_initial() -> Self {
        let turn = Player::Black;
        let board = [Square::Empty; BOARD_SQUARES];
        Self {
            turn,
            board,
            moves: precompute_moves(turn, board),
        }
    }

    pub fn othello_initial() -> Self {
        let turn = Player::Black;
        let mut board = [Square::Empty; BOARD_SQUARES];
        board[27] = Square::Placed(Player::White); // D4
        board[28] = Square::Placed(Player::Black); // E4
        board[35] = Square::Placed(Player::Black); // D5
        board[36] = Square::Placed(Player::White); // E5
        Self {
            turn,
            board,
            moves: precompute_moves(turn, board),
        }
    }

    pub fn random_state_after(n: u32) -> Self {
        let mut strategy = RandomMove::default();
        'outer: loop {
            let mut gs = Self::othello_initial();
            for _ in 0..n {
                if gs.moves.is_empty() {
                    continue 'outer;
                }
                gs = gs.make_move(strategy.decide(&gs));
            }
            return gs;
        }
    }

    pub const fn at(&self, position: Position) -> Square {
        self.board[position.index()]
    }

    pub fn discs_of(&self, player: Player) -> impl Iterator<Item = Position> + '_ {
        Position::all().filter(move |&pos| self.at(pos) == Square::Placed(player))
    }

    pub fn score_of(&self, player: Player) -> usize {
        match self.outcome() {
            Some(Outcome::Winner(p)) if p == player => {
                // counting empty squares for the winner
                BOARD_SQUARES - self.discs_of(player.opponent()).count()
            }
            Some(Outcome::Draw) => BOARD_SQUARES / 2,
            Some(Outcome::Winner(_)) | None => self.discs_of(player).count(),
        }
    }

    pub fn occupied_squares(&self) -> impl Iterator<Item = Position> + '_ {
        Position::all().filter(|&pos| matches!(self.at(pos), Square::Placed(_)))
    }

    pub fn moves(&self) -> &[Position] {
        &self.moves
    }

    fn pass_if_required(&mut self) {
        if self.moves().is_empty() {
            // No moves for opponent, pass
            self.turn = self.turn.opponent();
            let previous = mem::replace(&mut self.moves, precompute_moves(self.turn, self.board));
            if self.moves().is_empty() {
                // No moves again, game is over, correct the player
                self.turn = self.turn.opponent();
                self.moves = previous;
            }
        }
    }

    pub fn make_move(&self, position: Position) -> GameState {
        if !self.moves.contains(&position) {
            panic!("Invalid move!");
        }

        let mut next_state = (*self).clone();
        next_state.board[position.index()] = Square::Placed(self.turn);

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
                            .for_each(|p| next_state.board[p.index()] = Square::Placed(self.turn));
                        break;
                    }
                    Square::Empty => break,
                }
            }
        }

        next_state.turn = next_state.turn.opponent();
        next_state.moves = precompute_moves(next_state.turn, next_state.board);
        next_state.pass_if_required();
        next_state
    }

    pub fn outcome(&self) -> Option<Outcome> {
        if !self.moves().is_empty() {
            return None;
        }

        let black_discs = self.discs_of(Player::Black).count();
        let white_discs = self.discs_of(Player::White).count();
        Some(match black_discs.cmp(&white_discs) {
            Ordering::Less => Outcome::Winner(Player::White),
            Ordering::Equal => Outcome::Draw,
            Ordering::Greater => Outcome::Winner(Player::Black),
        })
    }

    pub fn from_board_string_unverified(board_str: &str) -> Option<GameState> {
        let board_str = strip_string(board_str);
        if board_str.len() != BOARD_SQUARES {
            return None;
        }

        let board = board_str
            .chars()
            .map(|c| match c {
                '0' => Square::Empty,
                '1' => Square::Placed(Player::Black),
                '2' => Square::Placed(Player::White),
                _ => unreachable!(), // strip_string should only leave 0, 1 and 2
            })
            .collect::<Vec<Square>>()
            .try_into()
            .unwrap();
        let turn = Player::Black;
        let mut result = GameState {
            board,
            turn,
            moves: precompute_moves(turn, board),
        };
        if result.occupied_squares().count() % 2 == 1 {
            result.turn = Player::White;
            result.moves = precompute_moves(result.turn, board);
        }
        result.pass_if_required();
        Some(result)
    }

    fn original_discs(&self) -> HashMap<Position, Player> {
        let mut result = HashMap::new();
        for position in self.occupied_squares() {
            let mut is_original = true;
            for dir in DIRECTIONS {
                let opp = (-dir.0, -dir.1);
                let in_dir = matches!(
                    position.offset(dir).map(|p| self.at(p)),
                    Some(Square::Placed(_))
                );
                let in_opp = matches!(
                    position.offset(opp).map(|p| self.at(p)),
                    Some(Square::Placed(_))
                );
                if in_dir && in_opp {
                    is_original = false;
                    break;
                }
            }
            if is_original {
                let Square::Placed(color) = self.at(position) else { unreachable!() };
                result.insert(position, color);
            }
        }
        result
    }

    pub fn verify_reachability(&self, timeout: Duration) -> Option<bool> {
        let start_time = Instant::now();

        let target_disc_set = self.occupied_squares().collect::<HashSet<Position>>();
        let original_disc_map = self.original_discs();

        let mut stack = Vec::from([GameState::reversi_initial()]);
        let mut visited = HashSet::new();
        while let Some(current) = stack.pop() {
            if current == *self {
                return Some(true);
            }

            if Instant::now() - start_time >= timeout {
                return None;
            }

            for &position in current.moves() {
                if !target_disc_set.contains(&position) {
                    continue;
                }
                if let Some(&color) = original_disc_map.get(&position) {
                    if color != current.turn {
                        continue;
                    }
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
        for row in 0..BOARD_SIDE as i32 {
            for col in 0..BOARD_SIDE as i32 {
                let position = p("A1").offset((col, row)).unwrap();
                let mut square_str = self.at(position).to_string();
                if self.moves.contains(&position) {
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
        let mut moves = gs.moves().to_vec();
        moves.sort_by_key(|p| p.index());
        assert_eq!(moves, expected);
    }

    #[test]
    fn reversi_earlygame() {
        let gs = GameState::reversi_initial();
        assert_eq!(gs.occupied_squares().count(), 0);
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
}

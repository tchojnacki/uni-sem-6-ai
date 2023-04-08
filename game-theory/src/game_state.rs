use crate::{
    player::Player,
    position::{p, Position, BOARD_SIDE, BOARD_SQUARES},
    square::Square,
    styles::{strip_string, EMPTY_BG, VALID_FG},
};
use colored::Colorize;
use std::{
    cmp::Ordering,
    collections::HashSet,
    fmt::{self, Display},
};

#[derive(PartialEq, Eq, Debug)]
pub enum Outcome {
    Winner(Player),
    Draw,
}

impl Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Outcome::Winner(p) => p.fmt(f),
            Outcome::Draw => write!(f, "{}", "Draw".purple()),
        }
    }
}

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

#[derive(Clone)]
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

    fn score_of(&self, player: Player) -> usize {
        match self.outcome() {
            Some(Outcome::Winner(p)) if p == player => {
                // counting empty squares for the winner
                BOARD_SQUARES - self.discs_of(player.opponent()).count()
            }
            Some(Outcome::Draw) => BOARD_SQUARES / 2,
            Some(Outcome::Winner(_)) | None => self.discs_of(player).count(),
        }
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
        if next_state.valid_moves().next().is_none() {
            // No moves for opponent, pass
            next_state.turn = next_state.turn.opponent();
            if next_state.valid_moves().next().is_none() {
                // No moves again, game is over, correct the player
                next_state.turn = next_state.turn.opponent();
            }
        }

        Some(next_state)
    }

    pub fn outcome(&self) -> Option<Outcome> {
        if self.valid_moves().next().is_some() {
            return None;
        }

        let black_discs = self.discs_of(Player::Black).count();
        let white_discs = self.discs_of(Player::White).count();
        Some(match black_discs.cmp(&white_discs) {
            Ordering::Greater => Outcome::Winner(Player::Black),
            Ordering::Less => Outcome::Winner(Player::White),
            Ordering::Equal => Outcome::Draw,
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

    fn assert_valid_moves(gs: &GameState, expected: &[Position]) {
        let mut moves = gs.valid_moves().collect::<Vec<_>>();
        moves.sort_by_key(|p| p.index());
        assert_eq!(moves, expected);
    }

    fn from_transcript(transcript: &'static str) -> GameState {
        let mut gs = GameState::othello_initial();
        for m in transcript
            .as_bytes()
            .chunks_exact(2)
            .map(|c| p(std::str::from_utf8(c).unwrap()))
        {
            assert_eq!(gs.outcome(), None);
            gs = gs.make_move(m).unwrap();
        }
        gs
    }

    fn assert_outcome(gs: &GameState, outcome: Outcome, black: usize, white: usize) {
        assert_eq!(gs.outcome(), Some(outcome));
        assert_eq!(gs.discs_of(Player::Black).count(), black);
        assert_eq!(gs.discs_of(Player::White).count(), white);
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
        assert_eq!(gs.score_of(gs.turn), 2);
        assert_eq!(gs.score_of(gs.turn.opponent()), 2);
        assert_valid_moves(&gs, &[p("D3"), p("C4"), p("F5"), p("E6")]);

        // From: https://www.eothello.com/game-rules
        let gs = gs.make_move(p("D3")).unwrap();
        assert_valid_moves(&gs, &[p("C3"), p("E3"), p("C5")]);

        let gs = gs.make_move(p("C5")).unwrap();
        assert_valid_moves(&gs, &[p("B6"), p("C6"), p("D6"), p("E6"), p("F6")]);
    }

    #[test]
    fn shortest_possible_game() {
        // Shortest possible game, Manubu Maruo 1975
        let gs = from_transcript("E6F4E3F6G5D6E7F5C5");
        assert_outcome(&gs, Outcome::Winner(Player::Black), 13, 0);
    }

    #[test]
    fn game_finished_in_draw() {
        // Draw game, Othello Quest Online 2018
        let gs = from_transcript("E6F4C3C4D3D6C5C6D7B5B6F7A6A5B4A7F3C8E8C7F6E7G8G6F8F5G4E3D2H3G5G3H4H5H7D8B8A4B3D1C1B1C2E1E2A3B7F2F1G1B2A2G2H6H2G7");
        assert_outcome(&gs, Outcome::Draw, 30, 30);
    }

    #[test]
    fn grand_prix_ghent_2017_hassan_verstuyft() {
        // Hassan 3 – 17 Verstuyft, European Grand Prix Ghent 2017
        let gs = from_transcript("D3E3F4G3F3C5H3F2C4C3E2E1B3H4H5A3");
        assert_outcome(&gs, Outcome::Winner(Player::White), 3, 17);
    }

    #[test]
    fn woc_ghent_2017_vecchi_nicolas() {
        // Vecchi 13 – 45 Nicolas, World Othello Championship Ghent 2017
        let gs = from_transcript("F5D6C5F6C4F4E6D7E7C6F7D8C8E8G5B8E3F8B6B5A6A4A5A7C7B4G6H6H7B3D3C3C2C1H5D2E2F1B1D1B2A3E1H4H3G4G3F3G1F2B7H2H1G7");
        assert_outcome(&gs, Outcome::Winner(Player::White), 13, 45);
    }

    #[test]
    fn woc_mito_2016_nagano_schotte() {
        // Nagano 50 - 14 Schotte, World Othello Championship Mito 2016
        let gs = from_transcript("F5F6E6F4G5E7E3F3C5D3G4G3G6C3D7C6C4D6B5B3B4H4C7D8H5H3F8F7H2A6A5H7F2C8B6A4B7E2G2F1B8A8A7H1H6G7G1D2C1C2D1E1H8G8E8B1A1B2A3A2");
        assert_outcome(&gs, Outcome::Winner(Player::Black), 50, 14);
    }

    #[test]
    fn woc_prague_2018_aunchulee_fukuchi() {
        // Aunchulee 30 - 34 Fukuchi, World Othello Championship Prague 2018
        let gs = from_transcript("D3C5F6F5E6E3C3F3C4B4B3D6C7C2D2C6A4D7G5F4D1G4F7E7G6H5E8C8D8F8E2F2B6A6B5A5A7A3A2F1E1H6G1B2G3B7H4B1A8B8G7H8H7G8A1H2C1H3G2H1");
        assert_outcome(&gs, Outcome::Winner(Player::White), 30, 34);
    }
}

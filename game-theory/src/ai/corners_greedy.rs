use super::strategy::Strategy;
use crate::{GameState, Position};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::fmt::{self, Display};

pub struct CornersGreedy {
    rng: StdRng,
}

impl Default for CornersGreedy {
    fn default() -> Self {
        Self {
            rng: SeedableRng::from_entropy(),
        }
    }
}

impl Display for CornersGreedy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(CornersGreedy))
    }
}

impl Strategy for CornersGreedy {
    fn decide(&mut self, gs: &GameState) -> Position {
        let moves = gs.moves().to_vec();
        let valid_corners = Position::corners()
            .filter(|&c| moves.contains(&c))
            .collect::<Vec<Position>>();

        if valid_corners.is_empty() {
            return *moves.choose(&mut self.rng).unwrap();
        }

        *valid_corners.choose(&mut self.rng).unwrap()
    }
}

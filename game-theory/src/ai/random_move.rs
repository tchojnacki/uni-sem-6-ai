use super::strategy::Strategy;
use crate::{GameState, Position};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use std::fmt::{self, Display};

pub struct RandomMove {
    rng: StdRng,
}

impl Default for RandomMove {
    fn default() -> Self {
        Self {
            rng: SeedableRng::from_entropy(),
        }
    }
}

impl Display for RandomMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(RandomMove))
    }
}

impl Strategy for RandomMove {
    fn decide(&mut self, gs: &GameState) -> Position {
        *gs.moves().choose(&mut self.rng).unwrap()
    }
}

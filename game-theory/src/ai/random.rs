use super::strategy::Strategy;
use crate::{GameState, Position};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

pub struct RandomAi {
    rng: StdRng,
}

impl Default for RandomAi {
    fn default() -> Self {
        Self {
            rng: SeedableRng::from_entropy(),
        }
    }
}

impl Strategy for RandomAi {
    fn decide(&mut self, gs: &GameState) -> Position {
        let moves = gs.valid_moves().collect::<Vec<Position>>();
        *moves.choose(&mut self.rng).unwrap()
    }
}

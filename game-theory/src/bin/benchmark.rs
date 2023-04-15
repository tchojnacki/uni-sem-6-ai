use game_theory::{
    ai::{RandomMove, Strategy},
    GameState,
};
use std::time::{Duration, Instant};

fn main() {
    let mut total = Duration::ZERO;
    let mut tries = 0;
    let mut strategy = RandomMove::default();
    for _ in 0..1000 {
        for n in 4..=60 {
            let gs = GameState::random_state_after(n);
            if gs.outcome().is_some() {
                continue;
            }
            let position = strategy.decide(&gs);
            let before = Instant::now();
            gs.make_move(position);
            total += before.elapsed();
            tries += 1;
        }
    }
    println!("{} ns", total.as_nanos() / tries);
}

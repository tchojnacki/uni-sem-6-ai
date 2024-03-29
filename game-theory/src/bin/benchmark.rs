use game_theory::{
    ai::{RandomMove, Strategy},
    game::GameState,
};
use std::time::{Duration, Instant};

fn main() {
    let mut total = Duration::ZERO;
    let mut tries = 0;
    let strategy = RandomMove::default();
    for _ in 0..1000 {
        for n in 1..=60 {
            let gs = GameState::random_state_between_inc(n, n);
            if gs.outcome().is_some() {
                continue;
            }
            let position = strategy.decide(&gs);
            let before = Instant::now();
            let _ = gs.make_move(position);
            total += before.elapsed();
            tries += 1;
        }
    }
    println!("{} ns", total.as_nanos() / tries);
}

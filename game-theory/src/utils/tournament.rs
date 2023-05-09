use crate::{
    ai::Strategy,
    game::{GameState, Outcome, Player},
};
use rand::{thread_rng, Rng};
use std::{
    sync::mpsc::{channel, Receiver},
    thread::{available_parallelism, scope},
    time::{Duration, Instant},
};

pub fn run_tournament<'a, F: Fn(usize) -> &'a (dyn Strategy) + Sync>(
    competitors: usize,
    timeout: Duration,
    selector: F,
) -> Receiver<(usize, usize, Outcome)> {
    let (tx, rx) = channel();
    let threads = available_parallelism().map(|n| n.get()).unwrap_or(1);
    let start = Instant::now();
    scope(|s| {
        for _ in 0..threads {
            let tx = tx.clone();
            let selector = &selector;
            s.spawn(move || {
                while start.elapsed() <= timeout {
                    let bi = thread_rng().gen_range(0..competitors);
                    let wi = thread_rng().gen_range(0..competitors);
                    let mut gs = GameState::random_state_between_inc(3, 5);
                    if gs.outcome().is_some() || bi == wi {
                        continue;
                    }

                    let bs = selector(bi);
                    let ws = selector(wi);

                    while gs.outcome().is_none() {
                        let position = match gs.turn() {
                            Player::Black => bs,
                            Player::White => ws,
                        }
                        .decide(&gs);

                        gs = gs.make_move(position);
                    }

                    tx.send((bi, wi, gs.outcome().unwrap())).unwrap();
                }
            });
        }
    });
    rx
}

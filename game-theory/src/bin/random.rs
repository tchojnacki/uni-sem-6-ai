use game_theory::GameState;
use rand::{thread_rng, Rng};

fn main() {
    let n = thread_rng().gen_range(0..=60);
    let gs = GameState::random_state_after(n);
    println!("{}", gs);
}

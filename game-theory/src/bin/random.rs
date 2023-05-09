use game_theory::game::GameState;

fn main() {
    println!("{}", GameState::random_state_between_inc(1, 60));
}

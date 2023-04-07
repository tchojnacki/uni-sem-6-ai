use crate::game_state::GameState;

mod game_state;
mod player;
mod position;
mod square;

fn main() {
    let gs = GameState::othello_initial();
    println!("{}", gs);
}

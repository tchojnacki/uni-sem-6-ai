use crate::game_state::{GameState, Position};

mod game_state;

fn main() {
    let gs = GameState::othello_initial();
    println!("{:?}", gs.valid_moves().collect::<Vec<Position>>())
}

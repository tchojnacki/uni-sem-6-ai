use crate::{game_state::GameState, position::Position};

mod game_state;
mod player;
mod position;
mod square;

fn main() {
    let gs = GameState::othello_initial();
    println!("{:?}", gs.valid_moves().collect::<Vec<Position>>())
}

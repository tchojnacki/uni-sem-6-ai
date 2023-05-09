use game_theory::{
    ai::{PlayerInput, Strategy},
    game::GameState,
};

fn main() {
    let mut gs = GameState::othello_initial();
    let strategy = PlayerInput::default();
    print!("{}", gs);
    while gs.outcome().is_none() {
        let position = strategy.decide(&gs);
        gs = gs.make_move(position);
        print!("{}", gs);
    }
}

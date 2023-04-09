use game_theory::{
    ai::{PlayerProvidedAi, Strategy},
    GameState,
};

fn main() {
    let mut gs = GameState::othello_initial();
    let mut strategy = PlayerProvidedAi::default();
    print!("{}", gs);
    while gs.outcome().is_none() {
        let position = strategy.decide(&gs);
        gs = gs.make_move(position).unwrap();
        print!("{}", gs);
    }
}

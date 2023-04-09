use game_theory::{GameState, Position};
use std::io::{stdin, stdout, Write};

fn main() {
    let mut gs = GameState::othello_initial();
    print!("{}", gs);
    while gs.outcome().is_none() {
        print!("Position: ");
        stdout().flush().unwrap();
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        buffer.make_ascii_uppercase();
        if let Some(position) = Position::from(buffer.trim()) {
            if let Some(next) = gs.make_move(position) {
                gs = next;
                print!("{}", gs);
            } else {
                println!("Illegal move!");
            }
        } else {
            println!("Invalid position notation!");
        }
    }
}

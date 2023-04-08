use game_theory::{GameState, Position};
use std::io::{stdin, stdout, Write};

pub fn main() {
    let mut gs = GameState::othello_initial();

    loop {
        print!("{}", gs);
        let mut buffer = String::new();
        print!("Position: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut buffer).unwrap();
        buffer.make_ascii_uppercase();
        if let Some(position) = Position::from(buffer.trim()) {
            if let Some(next) = gs.make_move(position) {
                gs = next;
            } else {
                println!("Illegal move!");
            }
        } else {
            println!("Invalid position notation!");
        }
    }
}

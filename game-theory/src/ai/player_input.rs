use super::strategy::Strategy;
use crate::game::{bitboard as bb, GameState, Position};
use std::{
    fmt::{self, Display},
    io::{stdin, stdout, Write},
};

#[derive(Default)]
pub struct PlayerInput;

impl Display for PlayerInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, stringify!(PlayerInput))
    }
}

impl Strategy for PlayerInput {
    fn decide(&self, gs: &GameState) -> Position {
        loop {
            print!("Position: ");
            stdout().flush().unwrap();

            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            buffer.make_ascii_uppercase();

            if let Some(position) = Position::from(buffer.trim()) {
                if bb::has(gs.move_bb(), position) {
                    return position;
                } else {
                    println!("Illegal move!");
                }
            } else {
                println!("Invalid position notation!");
            }
        }
    }
}

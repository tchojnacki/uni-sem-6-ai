use super::strategy::Strategy;
use crate::{bitboard::has, GameState, Position};
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
    fn decide(&mut self, gs: &GameState) -> Position {
        loop {
            print!("Position: ");
            stdout().flush().unwrap();

            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
            buffer.make_ascii_uppercase();

            if let Some(position) = Position::from(buffer.trim()) {
                if has(gs.move_bitboard(), position) {
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

use super::strategy::Strategy;
use crate::{GameState, Position};
use std::io::{stdin, stdout, BufRead, Write};

pub struct PlayerInput {
    input: Box<dyn BufRead>,
    output: Box<dyn Write>,
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            input: Box::new(stdin().lock()),
            output: Box::new(stdout().lock()),
        }
    }
}

impl Strategy for PlayerInput {
    fn decide(&mut self, gs: &GameState) -> Position {
        loop {
            write!(self.output, "Position: ").unwrap();
            self.output.flush().unwrap();

            let mut buffer = String::new();
            self.input.read_line(&mut buffer).unwrap();
            buffer.make_ascii_uppercase();

            if let Some(position) = Position::from(buffer.trim()) {
                if gs.is_valid(position) {
                    return position;
                } else {
                    writeln!(self.output, "Illegal move!").unwrap();
                }
            } else {
                writeln!(self.output, "Invalid position notation!").unwrap();
            }
        }
    }
}

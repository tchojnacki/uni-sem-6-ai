use crate::Player;
use colored::Colorize;
use std::fmt::{self, Display};

#[derive(PartialEq, Eq, Debug)]
pub enum Outcome {
    Winner(Player),
    Draw,
}

impl Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Outcome::Winner(p) => p.fmt(f),
            Outcome::Draw => write!(f, "{}", "Draw".purple()),
        }
    }
}

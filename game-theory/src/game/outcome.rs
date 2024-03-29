use super::Player;
use colored::Colorize;
use std::fmt::{self, Display};

#[must_use]
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

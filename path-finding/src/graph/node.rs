use crate::structs::{Pos, Stop, Time};
use smol_str::SmolStr;
use std::rc::Rc;

pub type NodeIndex = usize;

pub(super) struct Node {
    pub stop: Stop,
    pub time: Time,
    pub line: Option<SmolStr>,
}

impl Node {
    pub fn new(name: &Rc<str>, pos: Pos, time: Time, line: Option<SmolStr>) -> Self {
        Node {
            stop: Stop {
                name: name.clone(),
                pos,
            },
            time,
            line,
        }
    }

    pub fn is_line_express(&self) -> bool {
        if let Some(line) = self.line.as_ref().and_then(|l| l.chars().next()) {
            line.is_uppercase()
        } else {
            false
        }
    }
}

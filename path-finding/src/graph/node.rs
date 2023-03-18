use crate::structs::{Pos, Stop, Time};
use smol_str::SmolStr;
use std::rc::Rc;

pub type NodeIndex = usize;

/// Represents the passenger state in a BusNetwork state graph.
/// - stop - which Stop is passenger currently near
/// - time - what time is it currently
/// - line - whether the passenger is in a bus or waiting at the Stop
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

    /// Is line's name an uppercase letter - those lines are express lines.
    pub fn is_line_express(&self) -> bool {
        if let Some(line) = self.line.as_ref().and_then(|l| l.chars().next()) {
            line.is_uppercase()
        } else {
            false
        }
    }
}

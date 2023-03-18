use crate::structs::Pos;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub(crate) struct Stop {
    pub name: Rc<str>,
    pub pos: Pos,
}

impl Stop {
    /// Is Stop's name uppercase (those stops are deemed more important than others).
    pub fn is_major(&self) -> bool {
        self.name.to_uppercase() == self.name.to_string()
    }
}

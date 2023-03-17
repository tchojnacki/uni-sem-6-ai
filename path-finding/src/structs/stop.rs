use crate::structs::Pos;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub(crate) struct Stop {
    pub name: Rc<str>,
    pub pos: Pos,
}

impl Stop {
    pub fn is_major(&self) -> bool {
        self.name.to_uppercase() == self.name.to_string()
    }
}

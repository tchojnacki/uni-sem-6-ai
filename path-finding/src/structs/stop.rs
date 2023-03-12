use crate::structs::Pos;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub(crate) struct Stop {
    pub name: Rc<str>,
    pub pos: Pos,
}

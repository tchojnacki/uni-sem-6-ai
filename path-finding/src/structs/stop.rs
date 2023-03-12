use super::pos::Pos;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub(crate) struct Stop {
    pub name: Rc<str>,
    pub pos: Pos,
}

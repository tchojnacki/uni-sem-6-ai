use crate::player::Player;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Square {
    Empty,
    Placed(Player),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn has_correct_mem_repr() {
        assert_eq!(mem::size_of::<Square>(), 1);
    }
}

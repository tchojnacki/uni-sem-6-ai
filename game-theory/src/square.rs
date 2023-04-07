use crate::player::Player;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Empty,
    Placed(Player),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn square_has_correct_repr() {
        assert_eq!(mem::size_of::<Square>(), 1);
    }
}

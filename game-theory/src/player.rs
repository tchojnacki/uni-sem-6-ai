#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Black = 1,
    White = 2,
}

impl Player {
    pub fn opposite(&self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn player_has_correct_repr() {
        assert_eq!(mem::size_of::<Player>(), 1);
        assert_eq!(Player::Black as usize, 1);
        assert_eq!(Player::White as usize, 2);
    }
}

use crate::{Position, BOARD_SQUARES};

pub type Bitboard = u64;

pub const EMPTY: Bitboard = 0x0000000000000000;
pub const FULL: Bitboard = !EMPTY;
pub const CENTER: Bitboard = 0x0000001818000000;
pub const CORNERS: Bitboard = 0x8100000000000081;
pub const EDGES: Bitboard = 0xFF818181818181FF;
pub const INTERNAL: Bitboard = !EDGES;
pub const OTHELLO_BLACK_START: Bitboard = 0x0000000810000000;
pub const OTHELLO_WHITE_START: Bitboard = 0x0000001008000000;

#[must_use]
pub const fn from_pos(position: Position) -> Bitboard {
    1 << position.index()
}

#[must_use]
pub const fn has(bitboard: Bitboard, position: Position) -> bool {
    bitboard & from_pos(position) != EMPTY
}

#[must_use]
pub fn positions(mut bb: Bitboard) -> Vec<Position> {
    let mut result = Vec::with_capacity(BOARD_SQUARES);
    let mut i = 0;
    while bb != EMPTY {
        if bb & 1 != EMPTY {
            result.push(Position::from_index(i));
        }
        bb >>= 1;
        i += 1;
    }
    result
}

#[must_use]
pub const fn valid_moves(current: Bitboard, opponent: Bitboard) -> Bitboard {
    let occupied = current | opponent;
    if occupied.count_ones() >= 4 {
        attack_fill(current, opponent)
    } else {
        CENTER & !occupied
    }
}

#[must_use]
pub fn potential_moves(current: Bitboard, opponent: Bitboard) -> Bitboard {
    neighbours(opponent) & !(current | opponent)
}

pub fn make_move(position: Position, current: &mut Bitboard, opponent: &mut Bitboard) {
    let position = from_pos(position);
    if valid_moves(*current, *opponent) & position == EMPTY {
        panic!("Invalid move!");
    }

    let flipped = all_flipped(position, *current, *opponent);
    *current |= position;
    *current |= flipped;
    *opponent ^= flipped;
}

#[must_use]
pub const fn diagonals(position: Position) -> [Bitboard; 4] {
    use dumb7fill::*;
    let bb = from_pos(position);
    [
        shift_nort(bb) | shift_sout(bb),
        shift_noea(bb) | shift_sowe(bb),
        shift_east(bb) | shift_west(bb),
        shift_soea(bb) | shift_nowe(bb),
    ]
}

#[must_use]
pub const fn neighbours(bb: Bitboard) -> Bitboard {
    use dumb7fill::*;
    shift_east(bb)
        | shift_sout(bb)
        | shift_west(bb)
        | shift_nort(bb)
        | shift_soea(bb)
        | shift_sowe(bb)
        | shift_nowe(bb)
        | shift_noea(bb)
}

#[must_use]
const fn all_flipped(position: Bitboard, current: Bitboard, opponent: Bitboard) -> Bitboard {
    use dumb7fill::*;
    fill_nort(position, opponent) & fill_sout(current, opponent)
        | fill_noea(position, opponent) & fill_sowe(current, opponent)
        | fill_east(position, opponent) & fill_west(current, opponent)
        | fill_soea(position, opponent) & fill_nowe(current, opponent)
        | fill_sout(position, opponent) & fill_nort(current, opponent)
        | fill_sowe(position, opponent) & fill_noea(current, opponent)
        | fill_west(position, opponent) & fill_east(current, opponent)
        | fill_nowe(position, opponent) & fill_soea(current, opponent)
}

#[must_use]
const fn attack_fill(current: Bitboard, opponent: Bitboard) -> Bitboard {
    use dumb7fill::*;
    !(current | opponent)
        & (shift_nort(fill_nort(current, opponent))
            | shift_noea(fill_noea(current, opponent))
            | shift_east(fill_east(current, opponent))
            | shift_soea(fill_soea(current, opponent))
            | shift_sout(fill_sout(current, opponent))
            | shift_sowe(fill_sowe(current, opponent))
            | shift_west(fill_west(current, opponent))
            | shift_nowe(fill_nowe(current, opponent)))
}

mod dumb7fill {
    // https://www.chessprogramming.org/Dumb7Fill
    // https://www.chessprogramming.org/General_Setwise_Operations

    use super::{Bitboard, FULL};
    use crate::BOARD_SIDE;

    // Compass rose is different than in chess, because Reversi ranks grow south, not north.
    const EAST: i32 = 1; // +1
    const SOUT: i32 = BOARD_SIDE as i32; // +8
    const WEST: i32 = -EAST; // -1
    const NORT: i32 = -SOUT; // -8
    const SOEA: i32 = SOUT + EAST; // +9
    const SOWE: i32 = SOUT + WEST; // +7
    const NOWE: i32 = NORT + WEST; // -9
    const NOEA: i32 = NORT + EAST; // -7

    const NOT_A_FILE: Bitboard = 0xFEFEFEFEFEFEFEFE;
    const NOT_H_FILE: Bitboard = 0x7F7F7F7F7F7F7F7F;

    #[must_use]
    const fn fill(mut gen: Bitboard, mut pro: Bitboard, dir: i32, mask: Bitboard) -> Bitboard {
        pro &= mask;
        gen = shift(gen, dir, pro);
        let mut result = gen;
        gen = shift(gen, dir, pro);
        result |= gen;
        gen = shift(gen, dir, pro);
        result |= gen;
        gen = shift(gen, dir, pro);
        result |= gen;
        gen = shift(gen, dir, pro);
        result |= gen;
        gen = shift(gen, dir, pro);
        (result | gen) & mask
    }

    #[must_use]
    pub const fn fill_east(gen: Bitboard, pro: Bitboard) -> Bitboard {
        fill(gen, pro, EAST, NOT_H_FILE)
    }

    #[must_use]
    pub const fn fill_sout(gen: Bitboard, pro: Bitboard) -> Bitboard {
        fill(gen, pro, SOUT, FULL)
    }

    #[must_use]
    pub const fn fill_west(gen: Bitboard, pro: Bitboard) -> Bitboard {
        fill(gen, pro, WEST, NOT_A_FILE)
    }

    #[must_use]
    pub const fn fill_nort(gen: Bitboard, pro: Bitboard) -> Bitboard {
        fill(gen, pro, NORT, FULL)
    }

    #[must_use]
    pub const fn fill_soea(gen: Bitboard, pro: Bitboard) -> Bitboard {
        fill(gen, pro, SOEA, NOT_H_FILE)
    }

    #[must_use]
    pub const fn fill_sowe(gen: Bitboard, pro: Bitboard) -> Bitboard {
        fill(gen, pro, SOWE, NOT_A_FILE)
    }

    #[must_use]
    pub const fn fill_nowe(gen: Bitboard, pro: Bitboard) -> Bitboard {
        fill(gen, pro, NOWE, NOT_A_FILE)
    }

    #[must_use]
    pub const fn fill_noea(gen: Bitboard, pro: Bitboard) -> Bitboard {
        fill(gen, pro, NOEA, NOT_H_FILE)
    }

    #[must_use]
    const fn shift(bitboard: Bitboard, by: i32, mask: Bitboard) -> Bitboard {
        mask & if by >= 0 {
            bitboard >> by
        } else {
            bitboard << -by
        }
    }

    #[must_use]
    pub const fn shift_east(bb: Bitboard) -> Bitboard {
        shift(bb, EAST, NOT_H_FILE)
    }

    #[must_use]
    pub const fn shift_sout(bb: Bitboard) -> Bitboard {
        shift(bb, SOUT, FULL)
    }

    #[must_use]
    pub const fn shift_west(bb: Bitboard) -> Bitboard {
        shift(bb, WEST, NOT_A_FILE)
    }

    #[must_use]
    pub const fn shift_nort(bb: Bitboard) -> Bitboard {
        shift(bb, NORT, FULL)
    }

    #[must_use]
    pub const fn shift_soea(bb: Bitboard) -> Bitboard {
        shift(bb, SOEA, NOT_H_FILE)
    }

    #[must_use]
    pub const fn shift_sowe(bb: Bitboard) -> Bitboard {
        shift(bb, SOWE, NOT_A_FILE)
    }

    #[must_use]
    pub const fn shift_nowe(bb: Bitboard) -> Bitboard {
        shift(bb, NOWE, NOT_A_FILE)
    }

    #[must_use]
    pub const fn shift_noea(bb: Bitboard) -> Bitboard {
        shift(bb, NOEA, NOT_H_FILE)
    }
}

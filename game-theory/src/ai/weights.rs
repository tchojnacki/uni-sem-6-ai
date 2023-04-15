use crate::BOARD_SQUARES;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

pub type WeightMatrix = [i32; BOARD_SQUARES];

pub fn weights_hash(weights: &[i32]) -> u8 {
    let mut hasher = DefaultHasher::new();
    weights.hash(&mut hasher);
    hasher.finish() as u8
}

#[rustfmt::skip]
pub const WEIGHTS_MAGGS: WeightMatrix = [
     64, -30,  10,   5,   5,  10, -30,  64,
    -30, -40,   2,   2,   2,   2, -40, -30,
     10,   2,   5,   1,   1,   5,   2,  10,
      5,   2,   1,   1,   1,   1,   2,   5,
      5,   2,   1,   1,   1,   1,   2,   5,
     10,   2,   5,   1,   1,   5,   2,  10,
    -30, -40,   2,   2,   2,   2, -40, -30,
     64, -30,  10,   5,   5,  10, -30,  64,
];

#[rustfmt::skip]
pub const WEIGHTS_SANNIDHANAM: WeightMatrix = [
     4, -3,  2,  2,  2,  2, -3,  4,
    -3, -4, -1, -1, -1, -1, -4, -3,
     2, -1,  1,  0,  0,  1, -1,  2,
     2, -1,  0,  1,  1,  0, -1,  2,
     2, -1,  0,  1,  1,  0, -1,  2,
     2, -1,  1,  0,  0,  1, -1,  2,
    -3, -4, -1, -1, -1, -1, -4, -3,
     4, -3,  2,  2,  2,  2, -3,  4,
];

#[rustfmt::skip]
pub const WEIGHTS_KORMAN: WeightMatrix = [
    20, -3, 11,  8,  8, 11, -3, 20,
    -3, -7, -4,  1,  1, -4, -7, -3,
    11, -4,  2,  2,  2,  2, -4, 11,
     8,  1,  2, -3, -3,  2,  1,  8,
     8,  1,  2, -3, -3,  2,  1,  8,
    11, -4,  2,  2,  2,  2, -4, 11,
    -3, -7, -4,  1,  1, -4, -7, -3,
    20, -3, 11,  8,  8, 11, -3, 20,
];

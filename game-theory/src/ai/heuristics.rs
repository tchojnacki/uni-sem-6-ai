use super::{
    minimax::{MAX_PLAYER, MIN_PLAYER},
    weights::{weights_hash, WeightMatrix, WEIGHTS_KORMAN},
};
use crate::{
    bitboard::{get_moves, neighbours, positions, square, CORNERS},
    GameState, BOARD_SQUARES,
};
use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

#[non_exhaustive]
pub enum Heuristic {
    /// - First mention: Rosenbloom 1982
    /// - AKA: p, coin party, piece difference
    MaximumDisc,
    /// - First mention: Stringham 1980
    MinimumDisc,
    /// - First mention: Maggs 1979
    /// - AKA: d, disk squares, weighted square, static heuristic
    Weighted(Box<WeightMatrix>),
    /// - First mention: TODO
    /// - AKA: c, corner occupancy, corners
    CornersOwned,
    /// - First mention: TODO
    /// - AKA: l, corner proximity
    CornerCloseness,
    /// - First mention: Rosenbloom 1982
    /// - AKA: m, actual mobility, current mobility
    Mobility,
    /// - First mention: Rosenbloom 1982
    /// - AKA: f
    FrontierDiscs,
    /// - First mention: Rosenbloom 1982
    /// - AKA: s
    Stability,
    /// - First mention: Korman 2003
    Korman,
}

impl Display for Heuristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Heuristic::MaximumDisc => write!(f, "MaxD"),
            Heuristic::MinimumDisc => write!(f, "MinD"),
            Heuristic::Weighted(weights) => write!(f, "W{:03}", weights_hash(weights.as_ref())),
            Heuristic::CornersOwned => write!(f, "CrOwn"),
            Heuristic::CornerCloseness => write!(f, "CrClose"),
            Heuristic::Mobility => write!(f, "Mob"),
            Heuristic::FrontierDiscs => write!(f, "Front"),
            Heuristic::Stability => write!(f, "Stab"),
            Heuristic::Korman => write!(f, "KORMAN"),
        }
    }
}

impl Heuristic {
    #[must_use]
    pub fn evaluate(&self, gs: &GameState) -> f64 {
        use Heuristic::*;
        let mut max_bb = gs.bitboard(MAX_PLAYER);
        let mut min_bb = gs.bitboard(MIN_PLAYER);
        match self {
            MaximumDisc => Self::ratio(gs.score_of(MAX_PLAYER), gs.score_of(MIN_PLAYER)),
            MinimumDisc => -&MaximumDisc.evaluate(gs),
            Weighted(weights) => {
                let mut total = 0.;
                for i in 0..BOARD_SQUARES {
                    total += ((max_bb & 1) as f64 - (min_bb & 1) as f64) * weights[i] as f64;
                    max_bb >>= 1;
                    min_bb >>= 1;
                }
                total
            }
            CornersOwned => {
                let max_corners = (gs.bitboard(MAX_PLAYER) & CORNERS).count_ones() as f64;
                let min_corners = (gs.bitboard(MIN_PLAYER) & CORNERS).count_ones() as f64;
                (max_corners - min_corners) / 4.
            }
            CornerCloseness => positions(CORNERS)
                .into_iter()
                .map(|p| {
                    let target = neighbours(square(p));
                    let max_sq = (target & gs.bitboard(MAX_PLAYER)).count_ones() as f64;
                    let min_sq = (target & gs.bitboard(MIN_PLAYER)).count_ones() as f64;
                    -0.125 * (max_sq - min_sq)
                })
                .sum(),
            Mobility => Self::ratio(
                get_moves(max_bb, min_bb).count_ones(),
                get_moves(min_bb, max_bb).count_ones(),
            ),
            FrontierDiscs => {
                let empty_bb = !(max_bb | min_bb);
                -Self::ratio(
                    (neighbours(max_bb) & empty_bb).count_ones(),
                    (neighbours(min_bb) & empty_bb).count_ones(),
                )
            }
            Stability => {
                // TODO
                0.
            }
            Korman => {
                // Weights taken from "Playing Othello with Artificial Intelligence", M. Korman 2003
                Self::linear_combination(
                    gs,
                    &[
                        (801.724, CornersOwned),
                        (382.026, CornerCloseness),
                        (78.922, Mobility),
                        (10., MaximumDisc),
                        (0.1, Weighted(Box::new(WEIGHTS_KORMAN))),
                        (74.396, FrontierDiscs),
                        (100., Stability),
                    ],
                )
            }
        }
    }

    fn ratio<F: Into<f64>>(max: F, min: F) -> f64 {
        let max = max.into();
        let min = min.into();
        match max.partial_cmp(&min).unwrap() {
            Ordering::Less => -min / (max + min),
            Ordering::Equal => 0.,
            Ordering::Greater => max / (max + min),
        }
    }

    fn linear_combination(gs: &GameState, factors: &[(f64, Heuristic)]) -> f64 {
        factors.iter().map(|(w, h)| w * h.evaluate(gs)).sum()
    }
}

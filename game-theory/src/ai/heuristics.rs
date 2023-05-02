use super::{
    minimax::{MAX_PLAYER, MIN_PLAYER},
    weights::{WeightMatrix, WEIGHTS_KORMAN, WEIGHTS_MAGGS, WEIGHTS_SANNIDHANAM},
};
use crate::{
    bitboard::{
        diagonals, has, neighbours, positions, potential_moves, square, valid_moves, Bitboard,
        CORNERS, EDGES, EMPTY, INTERNAL,
    },
    GameState, Player, BOARD_SQUARES,
};
use std::{
    cmp::Ordering,
    collections::HashSet,
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
    Weighted(&'static str, &'static WeightMatrix),
    /// - First mention: Korman 2003
    /// - AKA: c, corner occupancy, corners
    CornersOwned,
    /// - First mention: Korman 2003
    /// - AKA: l, corner proximity
    CornerCloseness,
    /// - First mention: Rosenbloom 1982
    /// - AKA: m, mobility, actual mobility
    CurrentMobility,
    /// - First mention: Rosenbloom 1982
    PotentialMobility,
    /// - First mention: Rosenbloom 1982
    /// - AKA: f
    FrontierDiscs,
    /// - First mention: Rosenbloom 1982
    /// - AKA: s, stability
    InternalStability,
    /// - First mention: Rosenbloom 1982
    EdgeStability,
    /// - First mention: Rosenbloom 1982
    Iago,
    /// - First mention: Korman 2003
    Korman,
}

impl Display for Heuristic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Heuristic::*;
        match self {
            MaximumDisc => write!(f, "MaxD"),
            MinimumDisc => write!(f, "MinD"),
            Weighted(name, _) => write!(f, "W({name})"),
            CornersOwned => write!(f, "CrOwn"),
            CornerCloseness => write!(f, "CrCls"),
            CurrentMobility => write!(f, "CurMob"),
            PotentialMobility => write!(f, "PotMob"),
            FrontierDiscs => write!(f, "Front"),
            InternalStability => write!(f, "InStab"),
            EdgeStability => write!(f, "EdStab"),
            Iago => write!(f, "IAGO"),
            Korman => write!(f, "KORMAN"),
        }
    }
}

impl Heuristic {
    pub const W_MAGGS: Heuristic = Heuristic::Weighted("MAGGS", &WEIGHTS_MAGGS);
    pub const W_SANNIDHANAM: Heuristic = Heuristic::Weighted("SANNIDHANAM", &WEIGHTS_SANNIDHANAM);
    pub const W_KORMAN: Heuristic = Heuristic::Weighted("KORMAN", &WEIGHTS_KORMAN);

    #[must_use]
    pub fn evaluate(&self, gs: &GameState) -> f64 {
        use Heuristic::*;
        let mut max_bb = gs.bitboard(MAX_PLAYER);
        let mut min_bb = gs.bitboard(MIN_PLAYER);
        match self {
            MaximumDisc => Self::ratio(gs.score_of(MAX_PLAYER), gs.score_of(MIN_PLAYER)),
            MinimumDisc => -&MaximumDisc.evaluate(gs),
            Weighted(_, weights) => {
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
            CurrentMobility => Self::ratio(
                valid_moves(max_bb, min_bb).count_ones(),
                valid_moves(min_bb, max_bb).count_ones(),
            ),
            PotentialMobility => Self::ratio(
                potential_moves(max_bb, min_bb).count_ones(),
                potential_moves(min_bb, max_bb).count_ones(),
            ),
            FrontierDiscs => {
                let empty_bb = !(max_bb | min_bb);
                -Self::ratio(
                    (neighbours(max_bb) & empty_bb).count_ones(),
                    (neighbours(min_bb) & empty_bb).count_ones(),
                )
            }
            InternalStability => {
                let stable = stable_bb(gs) & INTERNAL;
                Self::ratio(
                    (stable & max_bb).count_ones(),
                    (stable & min_bb).count_ones(),
                )
            }
            EdgeStability => {
                let stable = stable_bb(gs) & EDGES;
                Self::ratio(
                    (stable & max_bb).count_ones(),
                    (stable & min_bb).count_ones(),
                )
            }
            Iago => Self::linear_combination(
                gs, // Weights from: Rosenbloom 1982
                &[
                    (esac(gs.move_number()), EdgeStability),
                    (36., InternalStability),
                    (cmac(gs.move_number()), CurrentMobility),
                    (99., PotentialMobility),
                ],
            ),
            Korman => Self::linear_combination(
                gs, // Weights from: Korman 2003
                &[
                    (801.724, CornersOwned),
                    (382.026, CornerCloseness),
                    (78.922, CurrentMobility),
                    (10., MaximumDisc),
                    (0.1, Self::W_KORMAN),
                    (74.396, FrontierDiscs),
                    (100., InternalStability),
                ],
            ),
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

#[must_use]
fn esac(move_number: i32) -> f64 {
    assert!((1..=60).contains(&move_number));
    312. + 6.24 * move_number as f64
}

#[must_use]
fn cmac(move_number: i32) -> f64 {
    assert!((1..=60).contains(&move_number));
    if move_number <= 25 {
        50. + 2. * move_number as f64
    } else {
        75. + move_number as f64
    }
}

#[must_use]
pub fn stable_bb(gs: &GameState) -> Bitboard {
    let mut queue = Vec::new();
    let mut visited = HashSet::new();
    let mut stable = EMPTY;

    let occupied = gs.bitboard(Player::Black) | gs.bitboard(Player::White);

    let corners = occupied & CORNERS;
    stable |= corners;
    queue.extend(positions(corners));

    while let Some(source) = queue.pop() {
        if visited.contains(&source) {
            continue;
        }
        visited.insert(source);

        for pos in positions(neighbours(square(source))) {
            let mut is_stable = true;
            for line in diagonals(pos) {
                let neighbours = positions(line);
                if neighbours.len() == 2
                    && neighbours
                        .into_iter()
                        .all(|n| gs.at(n) != gs.at(pos) || !has(stable, n))
                {
                    is_stable = false;
                }
            }
            if is_stable {
                stable |= square(pos);
                queue.push(pos);
            }
        }
    }

    stable
}

use super::weights::{WeightMatrix, WEIGHTS_KORMAN, WEIGHTS_MAGGS, WEIGHTS_VAISHU_MUTHU};
use crate::game::{
    bitboard::{self as bb, Bitboard},
    GameState, Outcome, Player, BOARD_SQUARES,
};
use std::{
    cmp::Ordering,
    collections::{hash_map::DefaultHasher, HashSet},
    fmt::{self, Display},
    hash::{Hash, Hasher},
};

pub const MAX_PLAYER: Player = Player::Black;
pub const MIN_PLAYER: Player = Player::White;

impl Outcome {
    #[must_use]
    pub const fn evaluate(&self) -> f64 {
        match self {
            Outcome::Winner(MAX_PLAYER) => f64::INFINITY,
            Outcome::Winner(MIN_PLAYER) => f64::NEG_INFINITY,
            Outcome::Draw => 0.,
        }
    }
}

pub const LINEAR_WEIGHT_LEN: usize = 14;
const LINEAR_COMPONENTS: [Heuristic; LINEAR_WEIGHT_LEN / 2] = [
    Heuristic::MaximumDisc,
    Heuristic::CornersOwned,
    Heuristic::CornerCloseness,
    Heuristic::CurrentMobility,
    Heuristic::PotentialMobility,
    Heuristic::InternalStability,
    Heuristic::EdgeStability,
];

pub fn linear_hash(weights: &[f64; LINEAR_WEIGHT_LEN]) -> u8 {
    let mut state = DefaultHasher::new();
    for w in weights {
        w.to_bits().hash(&mut state);
    }
    state.finish() as u8
}

#[non_exhaustive]
#[derive(Debug, Clone)]
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
    /// - AKA: f, frontier discs
    PotentialMobility,
    /// - First mention: Rosenbloom 1982
    InternalStability,
    /// - First mention: Rosenbloom 1982
    EdgeStability,
    /// - First mention: Rosenbloom 1982 tylko jednego sąsiada.
    /// - AKA: s
    Stability,
    /// - First mention: Rosenbloom 1982
    LinearEquations(Box<[f64; LINEAR_WEIGHT_LEN]>),
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
            InternalStability => write!(f, "InStab"),
            EdgeStability => write!(f, "EdStab"),
            Stability => write!(f, "Stab"),
            LinearEquations(m_and_b) => write!(f, "LinEq({:03})", linear_hash(m_and_b)),
            Iago => write!(f, "IAGO"),
            Korman => write!(f, "KORMAN"),
        }
    }
}

impl Heuristic {
    pub const W_MAGGS: Self = Self::Weighted("MAGGS", &WEIGHTS_MAGGS);
    pub const W_VAISHU_MUTHU: Self = Self::Weighted("VAISHU&MUTHU", &WEIGHTS_VAISHU_MUTHU);
    pub const W_KORMAN: Self = Self::Weighted("KORMAN", &WEIGHTS_KORMAN);
    
    #[rustfmt::skip]
    #[must_use]
    pub fn le051() -> Self { Self::LinearEquations(Box::new([-0.064,-0.065,0.106,0.746,0.928,0.101,0.183,0.183,0.427,1.047,0.093,0.897,1.096,0.707])) }
    
    #[rustfmt::skip]
    #[must_use]
    pub fn le064() -> Self { Self::LinearEquations(Box::new([-0.088,-1.444,0.029,0.857,1.546,1.019,0.088,0.985,0.064,0.307,0.457,0.013,0.908,0.133])) }
 
    #[rustfmt::skip]
    #[must_use]
    pub fn le148() -> Self { Self::LinearEquations(Box::new([-0.198,0.553,0.755,0.602,0.345,-1.263,0.445,0.253,0.305,0.659,0.545,-0.061,0.782,-0.200])) }
   
    #[rustfmt::skip]
    #[must_use]
    pub fn le162() -> Self { Self::LinearEquations(Box::new([-0.250,-1.468,1.044,0.828,1.556,-1.389,0.625,0.700,0.023,0.347,0.246,0.053,1.078,0.325])) }

    #[rustfmt::skip]
    #[must_use]
    pub fn le215() -> Self { Self::LinearEquations(Box::new([-0.192,0.677,0.495,0.147,1.286,-0.197,0.545,0.307,0.128,0.666,0.596,-0.502,0.846,-0.485])) }

    #[must_use]
    pub fn evaluate(&self, gs: &GameState) -> f64 {
        use Heuristic::*;
        let mut max_bb = gs.bb_of(MAX_PLAYER);
        let mut min_bb = gs.bb_of(MIN_PLAYER);
        match self {
            MaximumDisc => Self::ratio(gs.score_of(MAX_PLAYER), gs.score_of(MIN_PLAYER)),
            MinimumDisc => -&MaximumDisc.evaluate(gs),
            Weighted(_, weights) => {
                let mut total = 0.;
                let mut max = 0.;
                for i in 0..BOARD_SQUARES {
                    total += ((max_bb & 1) as f64 - (min_bb & 1) as f64) * weights[i] as f64;
                    max += weights[i].max(0) as f64;
                    max_bb >>= 1;
                    min_bb >>= 1;
                }
                total / max
            }
            CornersOwned => {
                let max_corners = (gs.bb_of(MAX_PLAYER) & bb::CORNERS).count_ones() as f64;
                let min_corners = (gs.bb_of(MIN_PLAYER) & bb::CORNERS).count_ones() as f64;
                (max_corners - min_corners) / 4.
            }
            CornerCloseness => bb::positions(bb::CORNERS)
                .into_iter()
                .map(|p| {
                    if bb::has(gs.empty_bb(), p) {
                        let target = bb::neighbours(bb::from_pos(p));
                        let max_sq = (target & gs.bb_of(MAX_PLAYER)).count_ones() as f64;
                        let min_sq = (target & gs.bb_of(MIN_PLAYER)).count_ones() as f64;
                        -0.125 * (max_sq - min_sq)
                    } else {
                        0.
                    }
                })
                .sum(),
            CurrentMobility => Self::ratio(
                bb::valid_moves(max_bb, min_bb).count_ones(),
                bb::valid_moves(min_bb, max_bb).count_ones(),
            ),
            PotentialMobility => -Self::ratio(
                (bb::neighbours(max_bb) & gs.empty_bb()).count_ones(),
                (bb::neighbours(min_bb) & gs.empty_bb()).count_ones(),
            ),
            InternalStability => Self::stability_ratio(gs, bb::INTERNAL),
            EdgeStability => Self::stability_ratio(gs, bb::EDGES),
            Stability => Self::stability_ratio(gs, bb::FULL),
            LinearEquations(m_and_b) => Self::weighted_average(
                gs,
                &m_and_b
                    .chunks(2)
                    .map(|chunk| match chunk {
                        [m, b] => m * gs.move_number() as f64 + b,
                        _ => unreachable!(),
                    })
                    .zip(LINEAR_COMPONENTS)
                    .collect::<Vec<_>>(),
            ),
            Iago => Self::weighted_average(
                gs, // Weights from: Rosenbloom 1982
                &[
                    (Self::esac(gs.move_number()), EdgeStability),
                    (36., InternalStability),
                    (Self::cmac(gs.move_number()), CurrentMobility),
                    (99., PotentialMobility),
                ],
            ),
            Korman => Self::weighted_average(
                gs, // Weights from: Korman 2003
                &[
                    (802., CornersOwned),
                    (382., CornerCloseness),
                    (79., CurrentMobility),
                    (10., MaximumDisc),
                    (26., Self::W_KORMAN),
                    (74., PotentialMobility),
                    (100., Stability),
                ],
            ),
        }
    }

    #[must_use]
    fn ratio(max: u32, min: u32) -> f64 {
        let max: f64 = max.into();
        let min: f64 = min.into();
        match max.partial_cmp(&min).unwrap() {
            Ordering::Less => -min / (max + min),
            Ordering::Equal => 0.,
            Ordering::Greater => max / (max + min),
        }
    }

    #[must_use]
    fn weighted_average(gs: &GameState, factors: &[(f64, Heuristic)]) -> f64 {
        factors.iter().map(|(w, h)| w * h.evaluate(gs)).sum::<f64>()
            / factors.iter().map(|(w, _)| w).sum::<f64>()
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
    fn stability_ratio(gs: &GameState, mask: Bitboard) -> f64 {
        let max_bb = gs.bb_of(MAX_PLAYER);
        let min_bb = gs.bb_of(MIN_PLAYER);
        let stable = Self::stable_bb(gs) & mask;
        Self::ratio(
            (stable & max_bb).count_ones(),
            (stable & min_bb).count_ones(),
        )
    }

    #[must_use]
    fn stable_bb(gs: &GameState) -> Bitboard {
        let mut queue = Vec::new();
        let mut visited = HashSet::new();
        let mut stable = bb::EMPTY;

        let corners = gs.occupied_bb() & bb::CORNERS;
        stable |= corners;
        queue.extend(bb::positions(corners));

        while let Some(source) = queue.pop() {
            if visited.contains(&source) {
                continue;
            }
            visited.insert(source);

            for pos in source.neighbours() {
                let mut is_stable = true;
                for line in bb::diagonals(pos) {
                    let neighbours = bb::positions(line);
                    if neighbours.len() == 2
                        && neighbours
                            .into_iter()
                            .all(|n| gs.at(n) != gs.at(pos) || !bb::has(stable, n))
                    {
                        is_stable = false;
                    }
                }
                if is_stable {
                    stable |= bb::from_pos(pos);
                    queue.push(pos);
                }
            }
        }

        stable
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Heuristic {
        fn arbitrary(gen: &mut quickcheck::Gen) -> Self {
            use Heuristic::*;
            gen.choose(&[
                MaximumDisc,
                MinimumDisc,
                Heuristic::W_MAGGS,
                Heuristic::W_VAISHU_MUTHU,
                Heuristic::W_KORMAN,
                CornersOwned,
                CornerCloseness,
                CurrentMobility,
                PotentialMobility,
                InternalStability,
                EdgeStability,
                Stability,
                Iago,
                Korman,
            ])
            .cloned()
            .unwrap()
        }
    }

    #[quickcheck]
    fn all_heuristics_are_normalized(heuristic: Heuristic, gs: GameState) -> bool {
        (-1. ..=1.).contains(&heuristic.evaluate(&gs))
    }

    #[quickcheck]
    fn ratio_is_normalized(max: u32, min: u32) -> bool {
        (-1. ..=1.).contains(&Heuristic::ratio(max, min))
    }

    #[quickcheck]
    fn esac_returns_nonnegative_coeffs(gs: GameState) -> bool {
        Heuristic::esac(gs.move_number()) >= 0.
    }

    #[quickcheck]
    fn cmac_returns_nonnegative_coeffs(gs: GameState) -> bool {
        Heuristic::cmac(gs.move_number()) >= 0.
    }
}

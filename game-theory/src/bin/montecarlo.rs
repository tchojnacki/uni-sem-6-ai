use game_theory::{
    ai::{
        weights::{WEIGHTS_KORMAN, WEIGHTS_MAGGS, WEIGHTS_SANNIDHANAM},
        AlphaBeta, CornersGreedy, FirstMove, Heuristic, Minimax, RandomMove, ScoreGreedy, Strategy,
    },
    GameState, Outcome, Player,
};
use rand::{seq::SliceRandom, thread_rng};
use std::iter::repeat;

const ELO_K: f64 = 32.;

fn elo_probability(loser: i32, winner: i32) -> f64 {
    1. / (1. + 10f64.powf((loser as f64 - winner as f64) / 400.))
}

fn elo_update(ratings: &mut [i32], wi: usize, li: usize) {
    let pw = elo_probability(ratings[li], ratings[wi]);
    let pl = elo_probability(ratings[wi], ratings[li]);
    ratings[wi] += (ELO_K * (1. - pw)) as i32;
    ratings[li] += (-pl * ELO_K) as i32;
}

fn run_tournament(strats: &mut [&mut dyn Strategy], rounds: usize) {
    let mut elo = vec![1000; strats.len()];

    let mut queue = Vec::with_capacity(strats.len().pow(2) * rounds);
    for bi in 0..strats.len() {
        for wi in 0..strats.len() {
            queue.extend(repeat((bi, wi)).take(rounds));
        }
    }
    queue.shuffle(&mut thread_rng());

    while let Some((bi, wi)) = queue.pop() {
        let mut gs = GameState::random_state_after(5);

        while gs.outcome().is_none() {
            let strat = &mut strats[match gs.turn() {
                Player::Black => bi,
                Player::White => wi,
            }];
            let position = strat.decide(&gs);
            gs = gs.make_move(position);
        }

        if let Outcome::Winner(winner) = gs.outcome().unwrap() {
            match winner {
                Player::Black => elo_update(&mut elo, bi, wi),
                Player::White => elo_update(&mut elo, wi, bi),
            }
        }
    }

    for i in 0..elo.len() {
        println!("{}: {} ELO", strats[i], elo[i])
    }
}

fn main() {
    println!("NAIVE STRATEGIES");
    run_tournament(
        &mut [
            &mut RandomMove::default(),
            &mut FirstMove::default(),
            &mut ScoreGreedy::default(),
            &mut CornersGreedy::default(),
        ],
        10000,
    );

    println!("MINIMAX VS ALPHA-BETA");
    run_tournament(
        &mut [
            &mut Minimax::new(Heuristic::MaximumDisc, 4),
            &mut AlphaBeta::new(Heuristic::MaximumDisc, 4),
        ],
        100,
    );

    println!("WEIGHT MATRIX COMPARISON");
    run_tournament(
        &mut [
            &mut AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_MAGGS)), 4),
            &mut AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_SANNIDHANAM)), 4),
            &mut AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_KORMAN)), 4),
        ],
        100,
    );

    println!("MAX DEPTH COMPARISON");
    run_tournament(
        &mut [
            &mut AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_KORMAN)), 1),
            &mut AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_KORMAN)), 2),
            &mut AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_KORMAN)), 3),
            &mut AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_KORMAN)), 4),
        ],
        500,
    );

    println!("FULL TOURNAMENT");
    run_tournament(
        &mut [
            &mut CornersGreedy::default(),
            &mut AlphaBeta::new(Heuristic::MaximumDisc, 4),
            &mut AlphaBeta::new(Heuristic::MinimumDisc, 4),
            &mut AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_KORMAN)), 4),
        ],
        100,
    );
}

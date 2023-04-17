use colored::Colorize;
use game_theory::{
    ai::{
        weights::{WEIGHTS_KORMAN, WEIGHTS_MAGGS, WEIGHTS_SANNIDHANAM},
        AlphaBeta, CornersGreedy, FirstMove, Heuristic, Minimax, RandomMove, ScoreGreedy, Strategy,
    },
    GameState, Outcome, Player,
};
use rand::{thread_rng, Rng};
use std::{
    sync::{mpsc::channel, Arc},
    thread::{available_parallelism, scope},
    time::{Duration, Instant},
};

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

fn run_tournament(name: &str, strats: &[&dyn Strategy], timeout: Duration) {
    let strat_count = strats.len();
    let strats = Arc::new(strats);

    println!("{}", name.bright_blue().bold());
    let start = Instant::now();

    let (tx, rx) = channel();
    scope(|s| {
        let threads = available_parallelism().map(|n| n.get()).unwrap_or(1);
        for _ in 0..threads {
            let tx = tx.clone();
            let strats = strats.clone();
            s.spawn(move || {
                while start.elapsed() <= timeout {
                    let bi = thread_rng().gen_range(0..strat_count);
                    let wi = thread_rng().gen_range(0..strat_count);
                    let mut gs = GameState::random_state_between(6, 8);
                    if gs.outcome().is_some() || bi == wi {
                        continue;
                    }

                    while gs.outcome().is_none() {
                        let strat = match gs.turn() {
                            Player::Black => strats[bi],
                            Player::White => strats[wi],
                        };
                        let position = strat.decide(&gs);
                        gs = gs.make_move(position);
                    }

                    tx.send((bi, wi, gs.outcome().unwrap())).unwrap();
                }
            });
        }
    });
    drop(tx);

    let mut ratings = vec![1000; strat_count];
    let mut games = vec![0; strat_count];
    let mut wins = vec![0; strat_count];
    let mut total_games = 0;
    while let Ok((bi, wi, outcome)) = rx.recv() {
        if let Outcome::Winner(winner) = outcome {
            match winner {
                Player::Black => {
                    elo_update(&mut ratings, bi, wi);
                    wins[bi] += 1;
                }
                Player::White => {
                    elo_update(&mut ratings, wi, bi);
                    wins[wi] += 1;
                }
            }
            games[bi] += 1;
            games[wi] += 1;
        }
        total_games += 1;
    }

    println!("Played {total_games} games!");

    let mut indices = (0..strat_count).collect::<Vec<_>>();
    indices.sort_by_key(|i| -ratings[*i]);
    for (num, i) in indices.into_iter().enumerate() {
        println!(
            "{:>2}. {:^25} {:>4} MMR, {:>4.1}% WR",
            num + 1,
            strats[i].to_string(),
            ratings[i],
            100. * wins[i] as f64 / games[i] as f64
        );
    }
}

fn main() {
    run_tournament(
        "NAIVE STRATEGIES",
        &[
            &RandomMove::default(),
            &FirstMove::default(),
            &ScoreGreedy::default(),
            &CornersGreedy::default(),
        ],
        Duration::from_secs(5),
    );

    run_tournament(
        "MINIMAX VS ALPHA-BETA",
        &[
            &Minimax::new(Heuristic::MaximumDisc, 3),
            &AlphaBeta::new(Heuristic::MaximumDisc, 3),
        ],
        Duration::from_secs(10),
    );

    run_tournament(
        "WEIGHT MATRIX COMPARISON",
        &[
            &AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_MAGGS)), 4),
            &AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_SANNIDHANAM)), 4),
            &AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_KORMAN)), 4),
        ],
        Duration::from_secs(10),
    );

    run_tournament(
        "MAX DEPTH COMPARISON",
        &[
            &AlphaBeta::new(Heuristic::Korman, 1),
            &AlphaBeta::new(Heuristic::Korman, 2),
            &AlphaBeta::new(Heuristic::Korman, 3),
            &AlphaBeta::new(Heuristic::Korman, 4),
            &AlphaBeta::new(Heuristic::Korman, 5),
        ],
        Duration::from_secs(30),
    );

    run_tournament(
        "BASIC HEURISTICS",
        &[
            &AlphaBeta::new(Heuristic::MaximumDisc, 4),
            &AlphaBeta::new(Heuristic::MinimumDisc, 4),
            &AlphaBeta::new(Heuristic::CornersOwned, 4),
            &AlphaBeta::new(Heuristic::CornerCloseness, 4),
            &AlphaBeta::new(Heuristic::Mobility, 4),
            &AlphaBeta::new(Heuristic::FrontierDiscs, 4),
            &AlphaBeta::new(Heuristic::Stability, 4),
        ],
        Duration::from_secs(10),
    );

    run_tournament(
        "FULL TOURNAMENT",
        &[
            &RandomMove::default(),
            &ScoreGreedy::default(),
            &CornersGreedy::default(),
            &AlphaBeta::new(Heuristic::MaximumDisc, 4),
            &AlphaBeta::new(Heuristic::CornersOwned, 4),
            &AlphaBeta::new(Heuristic::CornerCloseness, 4),
            &AlphaBeta::new(Heuristic::Mobility, 4),
            &AlphaBeta::new(Heuristic::Korman, 4),
        ],
        Duration::from_secs(300),
    );
}

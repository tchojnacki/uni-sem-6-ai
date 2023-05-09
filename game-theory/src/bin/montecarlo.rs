use colored::Colorize;
use game_theory::{
    ai::{
        AlphaBeta, CornersGreedy, FirstMove, Heuristic, Minimax, RandomMove, ScoreGreedy, Strategy,
    },
    game::{Outcome, Player},
    utils::{
        elo::{elo_update, INITIAL_ELO},
        tournament::run_tournament,
    },
};
use std::time::Duration;

fn calculate_ratings(name: &str, strats: &[&dyn Strategy], timeout: Duration) {
    println!("{}", name.bright_blue().bold());
    let rx = run_tournament(strats.len(), timeout, |i| strats[i]);

    let mut ratings = vec![INITIAL_ELO; strats.len()];
    let mut games = vec![0; strats.len()];
    let mut wins = vec![0; strats.len()];
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
    let mut indices = (0..strats.len()).collect::<Vec<_>>();
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
    calculate_ratings(
        "NAIVE STRATEGIES",
        &[
            &RandomMove::default(),
            &FirstMove::default(),
            &ScoreGreedy::default(),
            &CornersGreedy::default(),
        ],
        Duration::from_secs(1),
    );

    calculate_ratings(
        "MINIMAX VS ALPHA-BETA",
        &[
            &Minimax::new(Heuristic::MaximumDisc, 3),
            &AlphaBeta::new(Heuristic::MaximumDisc, 3),
        ],
        Duration::from_secs(10),
    );

    calculate_ratings(
        "WEIGHT MATRIX COMPARISON",
        &[
            &AlphaBeta::new(Heuristic::W_MAGGS, 4),
            &AlphaBeta::new(Heuristic::W_SANNIDHANAM, 4),
            &AlphaBeta::new(Heuristic::W_KORMAN, 4),
        ],
        Duration::from_secs(10),
    );

    calculate_ratings(
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

    calculate_ratings(
        "BASIC HEURISTICS",
        &[
            &AlphaBeta::new(Heuristic::MaximumDisc, 3),
            &AlphaBeta::new(Heuristic::MinimumDisc, 3),
            &AlphaBeta::new(Heuristic::CornersOwned, 3),
            &AlphaBeta::new(Heuristic::CornerCloseness, 3),
            &AlphaBeta::new(Heuristic::CurrentMobility, 3),
            &AlphaBeta::new(Heuristic::PotentialMobility, 3),
            &AlphaBeta::new(Heuristic::FrontierDiscs, 3),
            &AlphaBeta::new(Heuristic::InternalStability, 3),
            &AlphaBeta::new(Heuristic::EdgeStability, 3),
            &AlphaBeta::new(Heuristic::Stability, 3),
        ],
        Duration::from_secs(10),
    );

    calculate_ratings(
        "FULL TOURNAMENT",
        &[
            &RandomMove::default(),
            &CornersGreedy::default(),
            &AlphaBeta::new(Heuristic::EdgeStability, 4),
            &AlphaBeta::new(Heuristic::CornersOwned, 4),
            &AlphaBeta::new(Heuristic::CurrentMobility, 4),
            &AlphaBeta::new(Heuristic::Korman, 4),
            &AlphaBeta::new(Heuristic::Iago, 4),
            &AlphaBeta::new(Heuristic::lineq1(), 4),
            &AlphaBeta::new(Heuristic::lineq2(), 4),
            &AlphaBeta::new(Heuristic::lineq3(), 4),
        ],
        Duration::from_secs(300),
    );
}

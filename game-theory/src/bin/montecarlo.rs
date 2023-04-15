use game_theory::{
    ai::{
        weights::{WEIGHTS_KORMAN, WEIGHTS_MAGGS, WEIGHTS_SANNIDHANAM},
        AlphaBeta, CornersGreedy, FirstMove, Heuristic, Minimax, RandomMove, ScoreGreedy, Strategy,
    },
    GameState, Outcome, Player,
};

const ROUNDS: usize = 100;

fn main() {
    let strats: &mut [&mut dyn Strategy] = &mut [
        &mut RandomMove::default(),
        &mut FirstMove::default(),
        &mut ScoreGreedy::default(),
        &mut CornersGreedy::default(),
        &mut Minimax::new(Heuristic::MaximumDisc, 4),
        &mut AlphaBeta::new(Heuristic::MaximumDisc, 4),
        &mut AlphaBeta::new(Heuristic::MinimumDisc, 4),
        &mut AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_MAGGS)), 4),
        &mut AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_SANNIDHANAM)), 4),
        &mut AlphaBeta::new(Heuristic::Weighted(Box::new(WEIGHTS_KORMAN)), 4),
    ];

    let mut wins = vec![0; strats.len()];
    for bi in 0..strats.len() {
        for wi in 0..strats.len() {
            let mut results = [0; 3];
            for _ in 0..ROUNDS {
                let mut gs = GameState::random_state_after(5);

                while gs.outcome().is_none() {
                    let strat = &mut strats[match gs.turn() {
                        Player::Black => bi,
                        Player::White => wi,
                    }];
                    let position = strat.decide(&gs);
                    gs = gs.make_move(position);
                }

                results[match gs.outcome().unwrap() {
                    Outcome::Draw => 0,
                    Outcome::Winner(Player::Black) => 1,
                    Outcome::Winner(Player::White) => 2,
                }] += 1;
            }
            println!(
                "{} B {}-{} W {} ({} draws)",
                strats[bi], results[1], results[2], strats[wi], results[0]
            );
            wins[bi] += results[1];
            wins[wi] += results[2];
        }
    }

    for i in 0..wins.len() {
        println!(
            "{}: {:.1}% WR",
            strats[i],
            100. * wins[i] as f64 / (wins.len() * 2 * ROUNDS) as f64
        )
    }
}

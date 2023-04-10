use game_theory::{
    ai::{AlphaBeta, FirstMove, Greedy, Heuristic, Minimax, RandomMove, Strategy},
    GameState, Outcome, Player,
};

const ROUNDS: usize = 100;

fn main() {
    let strats: &mut [&mut dyn Strategy] = &mut [
        &mut FirstMove::default(),
        &mut Greedy::default(),
        &mut RandomMove::default(),
        &mut Minimax::new(Heuristic::MaximumDiscs, 4),
        &mut AlphaBeta::new(Heuristic::MaximumDiscs, 4),
    ];

    let mut wins = vec![0; strats.len()];
    for bi in 0..strats.len() {
        for wi in 0..strats.len() {
            let mut results = [0; 3];
            for _ in 0..ROUNDS {
                let mut gs = GameState::othello_initial();

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

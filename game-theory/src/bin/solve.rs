use colored::{ColoredString, Colorize};
use game_theory::{
    ai::{AlphaBeta, Heuristic, Minimax, Strategy},
    strip_string, GameState, Player, BOARD_SIDE, BOARD_SQUARES,
};
use once_cell::sync::Lazy;
use std::{
    io::stdin,
    time::{Duration, Instant},
};

const VERIFICATION_TIMEOUT: Duration = Duration::from_secs(5);

static INFO: Lazy<ColoredString> = Lazy::new(|| "INFO".bright_blue());
static OK: Lazy<ColoredString> = Lazy::new(|| "OK".bright_green());
static WARN: Lazy<ColoredString> = Lazy::new(|| "WARN".bright_yellow());
static ERROR: Lazy<ColoredString> = Lazy::new(|| "ERROR".bright_red());
static CRITICAL: Lazy<ColoredString> = Lazy::new(|| "CRITICAL".bright_yellow().on_red().bold());

fn verify_board(gs: &GameState) {
    println!("{} Verifying board reachability...", *INFO);
    let reachable = gs.verify_reachability(VERIFICATION_TIMEOUT);
    match reachable {
        Some(true) => println!("{} Board was verified to be reachable by legal moves.", *OK),
        Some(false) => println!("{} Board was confirmed to be unreachable!", *ERROR),
        None => println!(
            "{} Board reachability could not be verified in {} seconds!",
            *WARN,
            VERIFICATION_TIMEOUT.as_secs()
        ),
    }
    if reachable != Some(true) {
        println!(
            "{} Continuing program execution, however certain algorithms may behave incorrectly.",
            *WARN
        );
    }
}

fn main() {
    println!("Enter the board string:");
    let mut board_str = String::with_capacity(BOARD_SQUARES);
    while board_str.len() < BOARD_SQUARES {
        let mut line = String::with_capacity(BOARD_SIDE);
        stdin().read_line(&mut line).unwrap();
        board_str += &strip_string(&line);
    }

    let gs = GameState::from_board_str_unverified(&board_str);
    let Some(mut gs) = gs else {
        println!("{} Invalid board string! Aborting...", *CRITICAL);
        return;
    };
    println!("{} Recognized game state:", *INFO);
    print!("{gs}");
    verify_board(&gs);

    let black_strat = Minimax::new(Heuristic::MaximumDisc, 7);
    let white_strat = AlphaBeta::new(Heuristic::MinimumDisc, 7);

    let start = Instant::now();
    while gs.outcome().is_none() {
        let position = match gs.turn() {
            Player::Black => black_strat.decide(&gs),
            Player::White => white_strat.decide(&gs),
        };
        gs = gs.make_move(position);
    }
    let duration = start.elapsed();

    print!("{gs}");
    eprintln!(
        "Visited tree nodes: {:.1e} {} + {:.1e} {} = {:.1e} | Computation time: {} ms",
        black_strat.visited(),
        "B".bright_black().bold(),
        white_strat.visited(),
        "W".bright_white().bold(),
        black_strat.visited() + white_strat.visited(),
        duration.as_millis().to_string().bright_blue().bold()
    );
}

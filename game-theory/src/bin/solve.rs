use clap::{value_parser, Parser};
use colored::Colorize;
use game_theory::{
    game::{GameState, Player, BOARD_SIDE, BOARD_SQUARES},
    utils::{
        cli::{build_strategy, HeuristicArg, CRITICAL, ERROR, HEURISTIC_LIST, INFO, OK, WARN},
        styles::strip_string,
    },
};
use std::{
    io::stdin,
    time::{Duration, Instant},
};

const VERIFICATION_TIMEOUT: Duration = Duration::from_secs(5);

#[derive(Parser)]
#[clap(after_help = &*HEURISTIC_LIST)]
struct Args {
    /// Heuristic for player 1
    #[arg(
        long = "bh",
        default_value = "korman",
        hide_possible_values = true,
        help_heading = "Player 1"
    )]
    black_heuristic: HeuristicArg,

    /// Max recursion depth for player 1 in range 1..=10
    #[arg(long = "bd", default_value_t = 5, value_parser = value_parser!(u32).range(1..=10), help_heading = "Player 1")]
    black_depth: u32,

    /// Disable alpha-beta pruning for player 1 (use pure Minmax)
    #[arg(long = "bm", help_heading = "Player 1")]
    no_black_pruning: bool,

    /// Heuristic for player 2
    #[arg(
        long = "wh",
        default_value = "korman",
        hide_possible_values = true,
        help_heading = "Player 2"
    )]
    white_heuristic: HeuristicArg,

    /// Max recursion depth for player 2 in range 1..=10
    #[arg(long = "wd", default_value_t = 5, value_parser = value_parser!(u32).range(1..=10), help_heading = "Player 2")]
    white_depth: u32,

    /// Disable alpha-beta pruning for player 2 (use pure Minmax)
    #[arg(long = "wm", help_heading = "Player 2")]
    no_white_pruning: bool,

    /// Don't print the initial info
    #[arg(short = 'i', long)]
    no_initial: bool,

    /// Don't verify the given game state
    #[arg(short = 'v', long)]
    no_verification: bool,
}

fn board_prompt() -> Option<GameState> {
    println!("Enter the board string:");
    let mut board_str = String::with_capacity(BOARD_SQUARES);
    while board_str.len() < BOARD_SQUARES {
        let mut line = String::with_capacity(BOARD_SIDE);
        stdin().read_line(&mut line).unwrap();
        board_str += &strip_string(&line);
    }
    GameState::from_board_str_unverified(&board_str)
}

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
    let args = Args::parse();
    let Some(mut gs) = board_prompt() else {
        println!("{} Invalid board string! Aborting...", *CRITICAL);
        return;
    };

    let black_strat = build_strategy(
        args.black_heuristic,
        args.black_depth,
        args.no_black_pruning,
    );
    let white_strat = build_strategy(
        args.white_heuristic,
        args.white_depth,
        args.no_white_pruning,
    );

    if !args.no_initial {
        println!("{} Player 1: {}", *INFO, black_strat);
        println!("{} Player 2: {}", *INFO, white_strat);
        println!("{} Recognized game state:", *INFO);
        print!("{gs}");
    }

    if !args.no_verification {
        verify_board(&gs);
    }

    let start = Instant::now();
    while gs.outcome().is_none() {
        let position = match gs.turn() {
            Player::Black => black_strat.decide(&gs),
            Player::White => white_strat.decide(&gs),
        };
        gs = gs.make_move(position);
    }
    let duration = start.elapsed();

    println!("{} Solved board:", *OK);
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

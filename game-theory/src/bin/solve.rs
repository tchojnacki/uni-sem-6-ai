use clap::{value_parser, Parser, ValueEnum};
use colored::{ColoredString, Colorize};
use game_theory::{
    ai::{AlphaBeta, Heuristic, Minimax, TreeVisitingStrategy},
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

#[derive(Copy, Clone, ValueEnum)]
enum HeuristicArg {
    MaxDisc,
    MinDisc,
    WMaggs,
    WSannid,
    WKorman,
    CornOwn,
    CornClose,
    CurMob,
    PotMob,
    FrontDisc,
    IntStab,
    EdgeStab,
    Stab,
    Iago,
    Korman,
}

impl From<HeuristicArg> for Heuristic {
    fn from(value: HeuristicArg) -> Self {
        use {Heuristic as H, HeuristicArg as HA};
        match value {
            HA::MaxDisc => H::MaximumDisc,
            HA::MinDisc => H::MinimumDisc,
            HA::WMaggs => H::W_MAGGS,
            HA::WSannid => H::W_SANNIDHANAM,
            HA::WKorman => H::W_KORMAN,
            HA::CornOwn => H::CornersOwned,
            HA::CornClose => H::CornerCloseness,
            HA::CurMob => H::CurrentMobility,
            HA::PotMob => H::PotentialMobility,
            HA::FrontDisc => H::FrontierDiscs,
            HA::IntStab => H::InternalStability,
            HA::EdgeStab => H::EdgeStability,
            HA::Stab => H::Stability,
            HA::Iago => H::Iago,
            HA::Korman => H::Korman,
        }
    }
}

static HEURISTICS: Lazy<String> = Lazy::new(|| {
    let mut text = String::from("Available heuristics:\n");
    for chunk in HeuristicArg::value_variants()
        .iter()
        .filter_map(|v| v.to_possible_value())
        .collect::<Vec<_>>()
        .chunks(3)
    {
        for value in chunk {
            text += &format!("- {:32}", value.get_name());
        }
        text += "\n";
    }
    text
});

#[derive(Parser)]
#[clap(after_help = &*HEURISTICS)]
struct Args {
    /// Heuristic for player 1
    #[arg(long = "bh", default_value = "korman", hide_possible_values = true)]
    black_heuristic: HeuristicArg,

    /// Max recursion depth for player 1 in range 1..=10
    #[arg(long = "bd", default_value_t = 5, value_parser = value_parser!(u32).range(1..=10))]
    black_depth: u32,

    /// Disable alpha-beta pruning for player 1 (use pure Minmax)
    #[arg(long = "bm")]
    no_black_pruning: bool,

    /// Heuristic for player 2
    #[arg(long = "wh", default_value = "korman", hide_possible_values = true)]
    white_heuristic: HeuristicArg,

    /// Max recursion depth for player 2 in range 1..=10
    #[arg(long = "wd", default_value_t = 5, value_parser = value_parser!(u32).range(1..=10))]
    white_depth: u32,

    /// Disable alpha-beta pruning for player 2 (use pure Minmax)
    #[arg(long = "wm")]
    no_white_pruning: bool,

    /// Don't print the recognized game state
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

fn build_strategy(
    heuristic: HeuristicArg,
    depth: u32,
    no_pruning: bool,
) -> Box<dyn TreeVisitingStrategy> {
    let heuristic = heuristic.into();
    if no_pruning {
        Box::new(Minimax::new(heuristic, depth))
    } else {
        Box::new(AlphaBeta::new(heuristic, depth))
    }
}

fn main() {
    let args = Args::parse();
    let Some(mut gs) = board_prompt() else {
        println!("{} Invalid board string! Aborting...", *CRITICAL);
        return;
    };

    if !args.no_initial {
        println!("{} Recognized game state:", *INFO);
        print!("{gs}");
    }

    if !args.no_verification {
        verify_board(&gs);
    }

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

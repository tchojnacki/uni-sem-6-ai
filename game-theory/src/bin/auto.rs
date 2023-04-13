use colored::{ColoredString, Colorize};
use game_theory::{strip_string, GameState, BOARD_SIDE, BOARD_SQUARES};
use once_cell::sync::Lazy;
use std::{io::stdin, time::Duration};

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

    let gs = GameState::from_board_string_unverified(&board_str);
    let Some(gs) = gs else {
        println!("{} Invalid board string! Aborting...", *CRITICAL);
        return;
    };
    println!("{} Recognized game state:", *INFO);
    print!("{gs}");
    verify_board(&gs);
}

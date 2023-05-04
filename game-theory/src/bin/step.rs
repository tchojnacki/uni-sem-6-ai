use clap::{value_parser, ArgAction, Parser, ValueEnum};
use game_theory::{
    cli::{build_strategy, HeuristicArg, HEURISTIC_LIST},
    GameState, Player, Position,
};
use std::{io::stdin, thread, time::Duration};

#[derive(Clone, ValueEnum)]
enum PlayerArg {
    P1,
    P2,
}

impl From<PlayerArg> for Player {
    fn from(value: PlayerArg) -> Self {
        match value {
            PlayerArg::P1 => Player::Black,
            PlayerArg::P2 => Player::White,
        }
    }
}

#[derive(Parser)]
#[clap(after_help = &*HEURISTIC_LIST, disable_help_flag = true)]
struct Args {
    /// Player, for which turns should be played
    #[arg(short, long)]
    player: PlayerArg,

    /// Heuristic function for the player
    #[arg(short, long, default_value = "korman", hide_possible_values = true)]
    heuristic: HeuristicArg,

    /// Max recursion depth for the player
    #[arg(short, long, default_value_t = 5, value_parser = value_parser!(u32).range(1..=10))]
    depth: u32,

    /// Disable alpha-beta pruning for the player
    #[arg(short, long)]
    no_pruning: bool,

    /// Print help
    #[arg(long, action = ArgAction::Help)]
    help: Option<bool>,
}

fn read_position() -> Position {
    loop {
        let mut buffer = String::with_capacity(2);
        stdin().read_line(&mut buffer).unwrap();
        match Position::from(buffer.trim()) {
            Some(pos) => return pos,
            None => continue,
        }
    }
}

fn main() {
    let args = Args::parse();
    let my_player: Player = args.player.into();
    let strategy = build_strategy(args.heuristic, args.depth, args.no_pruning);

    let mut gs = GameState::othello_initial();
    if my_player == Player::White {
        eprintln!("{gs}");
    }

    while gs.outcome().is_none() {
        let in_control = gs.turn() == my_player;
        let position = if in_control {
            strategy.decide(&gs)
        } else {
            read_position()
        };

        gs = gs.make_move(position);

        if in_control {
            println!("{position}");
            eprintln!("{gs}");
        }

        thread::sleep(Duration::from_millis(100));
    }
}

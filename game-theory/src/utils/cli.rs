use crate::ai::{AlphaBeta, Heuristic, Minimax, TreeVisitingStrategy};
use clap::ValueEnum;
use colored::{ColoredString, Colorize};
use once_cell::sync::Lazy;

pub static INFO: Lazy<ColoredString> = Lazy::new(|| "INFO".bright_blue());
pub static OK: Lazy<ColoredString> = Lazy::new(|| "OK".bright_green());
pub static WARN: Lazy<ColoredString> = Lazy::new(|| "WARN".bright_yellow());
pub static ERROR: Lazy<ColoredString> = Lazy::new(|| "ERROR".bright_red());
pub static CRITICAL: Lazy<ColoredString> = Lazy::new(|| "CRITICAL".bright_yellow().on_red().bold());

#[derive(Clone, ValueEnum)]
pub enum HeuristicArg {
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

pub static HEURISTIC_LIST: Lazy<String> = Lazy::new(|| {
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

pub fn build_strategy(
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

use crate::graph::edge::Edge;
use colored::Colorize;
use std::{
    fmt::{self, Display},
    time::Duration,
};

pub struct Path<'a> {
    pub(super) edges: Vec<Edge<'a>>,
    pub(super) cost: u32,
    pub(super) runtime: Duration,
}

fn format_edge(edge: &Edge) -> String {
    match edge {
        Edge::Wait { .. } | Edge::Ride { .. } => format!("\t{}", edge.to_string().black()),
        Edge::Enter { .. } => format!("{}", edge.to_string().green().bold()),
        Edge::Leave { .. } => format!("{}", edge.to_string().red().bold()),
    }
}

impl Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let mut waits = Vec::new();

        for edge in &self.edges {
            if let Edge::Wait { .. } = edge {
                waits.push(edge);
                continue;
            } else {
                if !waits.is_empty() {
                    write!(f, "{}", format_edge(waits[0]))?;
                    if waits.len() > 2 {
                        writeln!(f, "\t{:>16}", "...".black())?;
                    }
                    write!(f, "{}", format_edge(waits[waits.len() - 1]))?;
                }
                waits.clear();
            }

            write!(f, "{}", format_edge(edge))?;
        }

        writeln!(
            f,
            "\nCost: {} min | Runtime: {} ms",
            self.cost.to_string().bright_blue().bold(),
            self.runtime.as_millis().to_string().bright_blue().bold()
        )
    }
}

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

impl Path<'_> {
    fn total_distance_km(&self) -> f32 {
        self.edges.iter().map(|e| e.distance_km()).sum()
    }

    fn total_time_min(&self) -> u32 {
        self.edges.iter().map(|e| e.time_min()).sum()
    }

    fn total_bus_changes(&self) -> u8 {
        self.edges
            .iter()
            .map(|e| e.bus_enter_count())
            .sum::<u8>()
            .saturating_sub(1)
    }
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
            "\nDistance: {:.3} km | Time: {} min | Changes: {}",
            self.total_distance_km(),
            self.total_time_min(),
            self.total_bus_changes()
        )?;
        writeln!(
            f,
            "Cost: {} | Runtime: {} ms",
            self.cost.to_string().bright_blue().bold(),
            self.runtime.as_millis().to_string().bright_blue().bold()
        )
    }
}

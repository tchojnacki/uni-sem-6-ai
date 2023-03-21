use crate::graph::{edge::Edge, state::Cost};
use colored::Colorize;
use std::{
    fmt::{self, Display},
    time::Duration,
};

pub struct Path<'bn, C: Cost> {
    pub(super) edges: Vec<Edge<'bn>>,
    pub(super) cost: C,
    pub(super) runtime: Duration,
}

impl<C: Cost> Path<'_, C> {
    fn total_distance_km(&self) -> f32 {
        self.edges.iter().map(|e| e.distance_km()).sum()
    }

    fn total_time_min(&self) -> u32 {
        self.edges.iter().map(|e| e.time_min()).sum()
    }

    fn total_buses(&self) -> u32 {
        self.edges.iter().map(|e| e.bus_count()).sum()
    }

    pub fn metrics(&self) -> String {
        let mut string = String::new();

        if cfg!(debug_assertions) {
            string += &format!(
                "Distance: {:.3} km | Time: {} min | Buses: {}\n",
                self.total_distance_km(),
                self.total_time_min(),
                self.total_buses()
            );
        }
        string += &format!(
            "Cost: {} | Runtime: {} ms\n",
            self.cost.to_string().bright_blue().bold(),
            self.runtime.as_millis().to_string().bright_blue().bold()
        );

        string
    }
}

impl<C: Cost> Display for Path<'_, C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        fn format_edge(edge: &Edge) -> String {
            match edge {
                Edge::Wait { .. } | Edge::Ride { .. } => format!("\t{}", edge.to_string().black()),
                Edge::Enter { .. } => format!("{}", edge.to_string().green().bold()),
                Edge::Leave { .. } => format!("{}", edge.to_string().red().bold()),
            }
        }

        let mut waits = Vec::new();

        for edge in &self.edges {
            if !cfg!(debug_assertions) && matches!(edge, Edge::Wait { .. } | Edge::Ride { .. }) {
                // Skip waiting and riding edges in release mode.
                continue;
            }

            if let Edge::Wait { .. } = edge {
                waits.push(edge);
                continue;
            } else {
                if !waits.is_empty() {
                    writeln!(f, "{}", format_edge(waits[0]))?;
                    if waits.len() > 2 {
                        writeln!(f, "\t{:>16}", "...".black())?;
                    }
                    writeln!(f, "{}", format_edge(waits[waits.len() - 1]))?;
                }
                waits.clear();
            }

            writeln!(f, "{}", format_edge(edge))?;
        }

        Ok(())
    }
}

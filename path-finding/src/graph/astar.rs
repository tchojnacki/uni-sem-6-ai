#![allow(clippy::type_complexity)]
use crate::{
    graph::{
        edge::Edge,
        node::Node,
        state::{Cost, State},
    },
    structs::Stop,
    BusNetwork, Path, Time,
};
use std::{
    collections::{BinaryHeap, HashMap},
    time::Instant,
};

fn astar<'bn, C, CF, HF>(
    bn: &'bn BusNetwork,
    start_name: &str,
    start_time: Time,
    end_name: &str,
    cost_fn: CF,
    heuristic_fn: HF,
) -> Option<Path<'bn, C>>
where
    C: Cost,
    CF: Fn(&Edge) -> C,
    HF: Fn(&Node, &Stop) -> C,
{
    // Based on the implementation provided through MS Teams.

    let instant = Instant::now();

    let start = bn.find_node_index(start_name, start_time);
    let end_stop = bn.find_stop(end_name);

    let mut costs = HashMap::with_capacity(bn.order());
    let mut parents = HashMap::with_capacity(bn.order());
    let mut queue = BinaryHeap::new();

    costs.insert(start, C::default());
    queue.push(State {
        cost: C::default(),
        node: start,
    });

    while let Some(State { node, .. }) = queue.pop() {
        if bn.is_valid_stop(node, end_name) {
            return Some(Path {
                edges: bn.reconstruct_edges(&parents, node),
                cost: costs[&node],
                runtime: instant.elapsed(),
            });
        }

        for neighbour in bn.neighbours(node) {
            let edge = Edge::from(bn.node(node), bn.node(neighbour));
            let new_cost = costs[&node] + cost_fn(&edge);

            if !costs.contains_key(&neighbour) || new_cost < costs[&neighbour] {
                let priority = new_cost + heuristic_fn(bn.node(neighbour), end_stop);
                costs.insert(neighbour, new_cost);
                parents.insert(neighbour, node);
                queue.push(State {
                    cost: priority,
                    node: neighbour,
                });
            }
        }
    }

    None
}

pub fn astar_time<'bn>(
    bn: &'bn BusNetwork,
    start_name: &str,
    start_time: Time,
    end_name: &str,
    expected_speed_kmph: f32,
) -> Option<Path<'bn, u32>> {
    let cost_fn = |edge: &Edge| edge.time_min();
    let heuristic_fn = |next: &Node, end: &Stop| {
        // v = s/t => t = s/v
        (next.stop.pos.distance_km(end.pos) / (expected_speed_kmph / 60.)) as u32
    };

    astar(bn, start_name, start_time, end_name, cost_fn, heuristic_fn)
}

pub enum StopHeuristic {
    Disabled,
    Distance { changes_per_km: f32 },
    StopNodes { weight: f32 },
    PreferMajorStops { penalty: u32 },
    AvoidExpressLines { penalty: u32 },
}

pub fn astar_buses<'bn>(
    bn: &'bn BusNetwork,
    start_name: &str,
    start_time: Time,
    end_name: &str,
    heuristic: StopHeuristic,
) -> Option<Path<'bn, u32>> {
    let cost_fn = |edge: &Edge| edge.bus_count();
    let heuristic_fn: Box<dyn Fn(&Node, &Stop) -> u32> = match heuristic {
        StopHeuristic::Disabled => Box::new(|_: &Node, _: &Stop| 0),
        StopHeuristic::Distance { changes_per_km } => Box::new(move |next: &Node, end: &Stop| {
            (next.stop.pos.distance_km(end.pos) * changes_per_km) as u32
        }),
        StopHeuristic::StopNodes { weight } => Box::new(move |next: &Node, _: &Stop| {
            (weight / bn.stop_nodes(&next.stop.name) as f32) as u32
        }),
        StopHeuristic::PreferMajorStops { penalty } => {
            Box::new(move |next: &Node, _: &Stop| if next.stop.is_major() { penalty } else { 0 })
        }
        StopHeuristic::AvoidExpressLines { penalty } => {
            Box::new(move |next: &Node, _: &Stop| if next.is_line_express() { penalty } else { 0 })
        }
    };

    astar(bn, start_name, start_time, end_name, cost_fn, heuristic_fn)
}

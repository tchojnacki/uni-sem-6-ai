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
    let cost = |edge: &Edge| edge.time_min();
    let heuristic = |next: &Node, end: &Stop| {
        // v = s/t => t = s/v
        (next.stop.pos.distance_km(end.pos) / (expected_speed_kmph / 60.)) as u32
    };

    astar(bn, start_name, start_time, end_name, cost, heuristic)
}

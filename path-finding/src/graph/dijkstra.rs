use crate::{
    graph::{edge::Edge, state::State},
    BusNetwork, Path, Time,
};
use std::{
    collections::{BinaryHeap, HashMap},
    time::Instant,
};

pub fn dijkstra_time<'bn>(
    bn: &'bn BusNetwork,
    start_name: &str,
    start_time: Time,
    end_name: &str,
) -> Option<Path<'bn, u32>> {
    // Based on the implementation provided through MS Teams.

    let instant = Instant::now();

    let start = bn.find_node_index(start_name, start_time)?;

    let mut costs = HashMap::with_capacity(bn.order());
    let mut parents = HashMap::with_capacity(bn.order());
    let mut queue = BinaryHeap::new();

    costs.insert(start, 0);
    queue.push(State {
        cost: 0,
        node: start,
    });

    while let Some(cur) = queue.pop() {
        if bn.is_valid_stop(cur.node, end_name) {
            return Some(Path {
                edges: bn.reconstruct_edges(&parents, cur.node),
                cost: cur.cost,
                runtime: instant.elapsed(),
            });
        } else if Some(&cur.cost) > costs.get(&cur.node) {
            continue;
        }

        for neighbour in bn.neighbours(cur.node) {
            let edge = Edge::from(bn.node(cur.node), bn.node(neighbour));
            let new_cost = cur.cost + edge.time_min();

            if !costs.contains_key(&neighbour) || new_cost < costs[&neighbour] {
                costs.insert(neighbour, new_cost);
                parents.insert(neighbour, cur.node);
                queue.push(State {
                    cost: new_cost,
                    node: neighbour,
                });
            }
        }
    }

    None
}

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
    let instant = Instant::now();

    let start = bn.find_node_index(start_name, start_time);

    let mut distances = HashMap::with_capacity(bn.order());
    let mut parents = HashMap::with_capacity(bn.order());
    let mut queue = BinaryHeap::new();

    let time_offset = bn.node(start).time - start_time;
    distances.insert(start, time_offset);
    queue.push(State {
        cost: time_offset,
        node: start,
    });

    while let Some(cur) = queue.pop() {
        if bn.is_valid_stop(cur.node, end_name) {
            let edges = bn.reconstruct_edges(&parents, cur.node);
            return Some(Path {
                edges,
                cost: cur.cost,
                runtime: instant.elapsed(),
            });
        } else if Some(&cur.cost) > distances.get(&cur.node) {
            continue;
        }

        for neighbour in bn.neighbours(cur.node) {
            let cost = Edge::from(bn.node(cur.node), bn.node(neighbour)).time_min();

            let new_cost = cur.cost + cost;
            if !distances.contains_key(&neighbour) || new_cost < distances[&neighbour] {
                distances.insert(neighbour, new_cost);
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

use crate::{graph::state::State, BusNetwork, Path, Time};
use std::{
    collections::{BinaryHeap, HashMap},
    time::Instant,
};

pub trait Dijkstra {
    fn dijkstra(&self, start_name: &str, start_time: Time, end_name: &str) -> Option<Path>;
}

impl Dijkstra for BusNetwork {
    fn dijkstra(&self, start_name: &str, start_time: Time, end_name: &str) -> Option<Path> {
        let instant = Instant::now();

        let start = self.find_node_index(start_name, start_time);

        let mut distances = HashMap::with_capacity(self.order());
        let mut parents = HashMap::with_capacity(self.order());
        let mut queue = BinaryHeap::new();

        let time_offset = self.node(start).time - start_time;
        distances.insert(start, time_offset);
        queue.push(State {
            cost: time_offset,
            node: start,
        });

        while let Some(cur) = queue.pop() {
            if self.is_valid_stop(cur.node, end_name) {
                let edges = self.reconstruct_edges(&parents, cur.node);
                return Some(Path {
                    edges,
                    cost: cur.cost,
                    runtime: instant.elapsed(),
                });
            } else if Some(&cur.cost) > distances.get(&cur.node) {
                continue;
            }

            for neighbour in self.neighbours(cur.node) {
                let cost = self.node(neighbour).time - self.node(cur.node).time;

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
}

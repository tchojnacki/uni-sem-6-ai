use crate::{
    graph::{edge::Edge, state::State},
    BusNetwork, Path, Time,
};
use std::{
    collections::{BinaryHeap, HashMap},
    ops::Add,
    time::Instant,
};

fn dijkstra<'bn, C: Add<Output = C> + Default + PartialOrd + Copy, F: Fn(&Edge) -> C>(
    bn: &'bn BusNetwork,
    start_name: &str,
    start_time: Time,
    end_name: &str,
    cost_fn: F,
) -> Option<Path<'bn, C>> {
    let instant = Instant::now();

    let start = bn.find_node_index(start_name, start_time);

    let mut distances = HashMap::with_capacity(bn.order());
    let mut parents = HashMap::with_capacity(bn.order());
    let mut queue = BinaryHeap::new();

    distances.insert(start, C::default());
    queue.push(State {
        cost: C::default(),
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
            let edge = Edge::from(bn.node(cur.node), bn.node(neighbour));
            let cost = cost_fn(&edge);

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

pub trait Dijkstra {
    fn dijkstra_time(
        &self,
        start_name: &str,
        start_time: Time,
        end_name: &str,
    ) -> Option<Path<u32>>;

    fn dijkstra_distance(
        &self,
        start_name: &str,
        start_time: Time,
        end_name: &str,
    ) -> Option<Path<f32>>;

    fn dijkstra_changes(
        &self,
        start_name: &str,
        start_time: Time,
        end_name: &str,
    ) -> Option<Path<u8>>;
}

impl Dijkstra for BusNetwork {
    fn dijkstra_time(
        &self,
        start_name: &str,
        start_time: Time,
        end_name: &str,
    ) -> Option<Path<u32>> {
        dijkstra(self, start_name, start_time, end_name, |e| e.time_min())
    }

    fn dijkstra_distance(
        &self,
        start_name: &str,
        start_time: Time,
        end_name: &str,
    ) -> Option<Path<f32>> {
        dijkstra(self, start_name, start_time, end_name, |e| e.distance_km())
    }

    fn dijkstra_changes(
        &self,
        start_name: &str,
        start_time: Time,
        end_name: &str,
    ) -> Option<Path<u8>> {
        dijkstra(self, start_name, start_time, end_name, |e| {
            e.bus_enter_count()
        })
    }
}

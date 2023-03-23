use crate::{
    graph::{
        edge::Edge,
        node::NodeIndex,
        state::{Cost, State},
    },
    BusNetwork, Time,
};
use rand::{seq::SliceRandom, thread_rng};
use std::collections::{BinaryHeap, HashMap, VecDeque};

#[derive(Clone)]
pub(super) struct Solution<'bn, C: Cost> {
    order: Vec<&'bn str>,
    cost: C,
}

impl<'bn, C: Cost> Solution<'bn, C> {
    pub fn mutate<CF: Fn(&Edge) -> C>(
        &self,
        ctx: &SolutionContext<C, CF>,
        i: usize,
        j: usize,
    ) -> Self {
        let mut order = self.order.clone();
        order.swap(i, j);
        let cost = ctx.calculate_cost(&order);
        Self { order, cost }
    }

    pub fn cost(&self) -> C {
        self.cost
    }
}

pub(super) struct SolutionContext<'bn, C: Cost, CF: Fn(&Edge) -> C> {
    bn: &'bn BusNetwork,
    start_name: &'bn str,
    start_time: Time,
    cost_fn: CF,
}

impl<'bn, C: Cost, CF: Fn(&Edge) -> C> SolutionContext<'bn, C, CF> {
    pub fn new(bn: &'bn BusNetwork, start_name: &'bn str, start_time: Time, cost_fn: CF) -> Self {
        Self {
            bn,
            start_name,
            start_time,
            cost_fn,
        }
    }

    pub fn aspiration_criteria(&self, stops: &[&str]) -> C {
        let mut total = C::default();
        for i in 0..stops.len() {
            for j in 0..stops.len() {
                let start = self.bn.find_node_index(stops[i], self.start_time).unwrap();
                let (_, cost, _) = self.dijkstra_helper(start, stops[j]);
                total = total + cost;
            }
        }
        total
    }

    pub fn initial_solution(&self, stops: &[&'bn str]) -> Solution<'bn, C> {
        let mut order = stops.to_vec();
        order.shuffle(&mut thread_rng());
        let cost = self.calculate_cost(&order);
        Solution { order, cost }
    }

    fn calculate_cost(&self, solution: &[&str]) -> C {
        let mut solution = solution.iter().copied().collect::<VecDeque<_>>();
        solution.push_back(self.start_name);

        let mut previous = self
            .bn
            .find_node_index(self.start_name, self.start_time)
            .unwrap();
        let mut total = C::default();

        while let Some(current) = solution.pop_front() {
            let (next, cost, _) = self.dijkstra_helper(previous, current);
            previous = next;
            total = total + cost;
        }

        total
    }

    pub fn reconstruct_edges(&self, solution: &Solution<'bn, C>) -> Vec<Edge<'bn>> {
        let mut solution = solution.order.iter().copied().collect::<VecDeque<_>>();
        solution.push_back(self.start_name);

        let mut previous = self
            .bn
            .find_node_index(self.start_name, self.start_time)
            .unwrap();
        let mut path = Vec::new();

        while let Some(current) = solution.pop_front() {
            let (next, _, part) = self.dijkstra_helper(previous, current);
            previous = next;
            path.extend(part);
        }

        path
    }

    fn dijkstra_helper(&self, start: NodeIndex, end_name: &str) -> (NodeIndex, C, Vec<Edge<'bn>>) {
        let mut costs = HashMap::with_capacity(self.bn.order());
        let mut parents = HashMap::with_capacity(self.bn.order());
        let mut queue = BinaryHeap::new();
        costs.insert(start, C::default());
        queue.push(State {
            cost: C::default(),
            node: start,
        });

        while let Some(cur) = queue.pop() {
            if self.bn.is_valid_stop(cur.node, end_name) {
                return (
                    cur.node,
                    cur.cost,
                    self.bn.reconstruct_edges(&parents, cur.node),
                );
            } else if Some(&cur.cost) > costs.get(&cur.node) {
                continue;
            }

            for neighbour in self.bn.neighbours(cur.node) {
                let edge = Edge::from(self.bn.node(cur.node), self.bn.node(neighbour));
                let new_cost = cur.cost + (self.cost_fn)(&edge);
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

        panic!("No path exists!");
    }
}

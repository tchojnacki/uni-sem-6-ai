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
    start_name: &'bn str,
    start_time: Time,
    order: Vec<&'bn str>,
    cost: C,
}

impl<'bn, C: Cost> Solution<'bn, C> {
    pub fn initial<FC>(
        start_name: &'bn str,
        start_time: Time,
        stops: &[&'bn str],
        bn: &BusNetwork,
        cost_fn: FC,
    ) -> Self
    where
        FC: Fn(&Edge) -> C,
    {
        let mut order = stops.to_vec();
        order.shuffle(&mut thread_rng());
        let cost = cost(bn, start_name, start_time, &order, cost_fn);
        Self {
            start_name,
            start_time,
            order,
            cost,
        }
    }

    pub fn mutate<FC>(&self, i: usize, j: usize, bn: &BusNetwork, cost_fn: FC) -> Self
    where
        FC: Fn(&Edge) -> C,
    {
        let mut order = self.order.clone();
        order.swap(i, j);
        let cost = cost(bn, self.start_name, self.start_time, &order, cost_fn);
        Self {
            order,
            cost,
            ..*self
        }
    }

    pub fn cost(&self) -> C {
        self.cost
    }

    pub fn aspiration_criteria<FC>(
        stops: &[&str],
        bn: &BusNetwork,
        start_time: Time,
        cost_fn: FC,
    ) -> C
    where
        FC: Fn(&Edge) -> C,
    {
        let mut aspiration_criteria = C::default();
        for i in 0..stops.len() {
            for j in 0..stops.len() {
                let start = bn.find_node_index(stops[i], start_time).unwrap();
                let (_, cost) = dijkstra_helper(bn, start, stops[j], &cost_fn);
                aspiration_criteria = aspiration_criteria + cost;
            }
        }
        aspiration_criteria
    }
}

fn dijkstra_helper<C, CF>(
    bn: &BusNetwork,
    start: NodeIndex,
    end_name: &str,
    cost_fn: CF,
) -> (NodeIndex, C)
where
    C: Cost,
    CF: Fn(&Edge) -> C,
{
    let mut costs = HashMap::with_capacity(bn.order());
    let mut queue = BinaryHeap::new();
    costs.insert(start, C::default());
    queue.push(State {
        cost: C::default(),
        node: start,
    });

    while let Some(cur) = queue.pop() {
        if bn.is_valid_stop(cur.node, end_name) {
            return (cur.node, cur.cost);
        } else if Some(&cur.cost) > costs.get(&cur.node) {
            continue;
        }

        for neighbour in bn.neighbours(cur.node) {
            let edge = Edge::from(bn.node(cur.node), bn.node(neighbour));
            let new_cost = cur.cost + cost_fn(&edge);
            if !costs.contains_key(&neighbour) || new_cost < costs[&neighbour] {
                costs.insert(neighbour, new_cost);
                queue.push(State {
                    cost: new_cost,
                    node: neighbour,
                });
            }
        }
    }

    unreachable!();
}

fn cost<C, CF>(
    bn: &BusNetwork,
    start_name: &str,
    start_time: Time,
    solution: &[&str],
    cost_fn: CF,
) -> C
where
    C: Cost,
    CF: Fn(&Edge) -> C,
{
    let mut path = solution.iter().copied().collect::<VecDeque<_>>();
    path.push_back(start_name);
    let mut previous = bn.find_node_index(start_name, start_time).unwrap();
    let mut total = C::default();

    while let Some(current) = path.pop_front() {
        let (next, cost) = dijkstra_helper(bn, previous, current, &cost_fn);
        previous = next;
        total = total + cost;
    }

    total
}

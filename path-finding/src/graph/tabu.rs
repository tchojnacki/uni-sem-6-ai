use super::node::NodeIndex;
use crate::{
    graph::{
        edge::Edge,
        state::{Cost, State},
    },
    BusNetwork, Path, Time,
};
use rand::{seq::SliceRandom, thread_rng};
use std::collections::{BinaryHeap, HashMap, VecDeque};

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
    println!("{:?} {}", bn.node(start).stop, end_name);
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

fn tabu<'bn, C, CF>(
    bn: &'bn BusNetwork,
    start_name: &str,
    start_time: Time,
    stops: &[&str],
    cost_fn: CF,
) -> Option<Path<'bn, C>>
where
    C: Cost,
    CF: Fn(&Edge) -> C,
{
    // Based on the implementation provided through MS Teams.

    let max_iterations = (1.1 * stops.len().pow(2) as f64).ceil() as usize;
    let mut turns_improved: usize = 0;
    let improve_thresh = 2 * (max_iterations as f64).sqrt().floor() as usize;
    let mut tabu_list = VecDeque::new();
    let tabu_tenure = stops.len();

    let mut aspiration_criteria = C::default();
    for i in 0..stops.len() {
        for j in 0..stops.len() {
            let start = bn.find_node_index(stops[i], start_time).unwrap();
            let (_, cost) = dijkstra_helper(bn, start, stops[j], &cost_fn);
            aspiration_criteria = aspiration_criteria + cost;
        }
    }

    let mut current_solution = Vec::from(stops);
    current_solution.shuffle(&mut thread_rng());
    let mut best_solution = current_solution.clone();
    let mut best_solution_cost = cost(bn, start_name, start_time, &best_solution, &cost_fn);

    for iteration in 0..max_iterations {
        if turns_improved > improve_thresh {
            break;
        }

        let mut best_neighbour = None;
        let mut best_neighbour_cost = None;
        let mut tabu_candidate = (0, 0);

        for i in 0..stops.len() {
            for j in i + 1..stops.len() {
                let mut neighbour = current_solution.clone();
                neighbour.swap(i, j);
                let neighbour_cost = cost(bn, start_name, start_time, &neighbour, &cost_fn);

                if (!tabu_list.contains(&(i, j)) || neighbour_cost < aspiration_criteria)
                    && (best_neighbour.is_none() || Some(neighbour_cost) < best_neighbour_cost)
                {
                    best_neighbour = Some(neighbour.clone());
                    best_neighbour_cost = Some(neighbour_cost);
                    tabu_candidate = (i, j);
                }
            }
        }

        if best_neighbour.is_some() {
            current_solution = best_neighbour.clone().unwrap();
            tabu_list.push_back(tabu_candidate);
            if tabu_list.len() > tabu_tenure {
                tabu_list.pop_front();
            }
            if best_neighbour_cost.unwrap() < best_solution_cost {
                best_solution = best_neighbour.clone().unwrap();
                best_solution_cost = best_neighbour_cost.unwrap();
                turns_improved = 0;
            } else {
                turns_improved += 1;
            }
        }

        println!(
            "Iteration {}: Best solution cost = {}",
            iteration, best_solution_cost
        );
    }

    println!("Best solution: {:?}", best_solution);
    println!("Best solution cost: {}", best_solution_cost);

    None
}

pub fn tabu_time<'bn>(
    bn: &'bn BusNetwork,
    start_name: &str,
    start_time: Time,
    stops: &[&str],
) -> Option<Path<'bn, u32>> {
    tabu(bn, start_name, start_time, stops, |edge| edge.time_min())
}

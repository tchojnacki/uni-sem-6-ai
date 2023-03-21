use crate::{
    graph::{edge::Edge, solution::SolutionContext, state::Cost},
    BusNetwork, Path, Time,
};
use colored::Colorize;
use rand::{thread_rng, Rng};
use std::{collections::VecDeque, time::Instant};

fn is_invalid(name: &str, bn: &BusNetwork) -> bool {
    bn.find_node_index(name, Time::new(0, 0)).is_none()
}

fn tabu<'bn, C, CF>(
    bn: &'bn BusNetwork,
    start_name: &'bn str,
    start_time: Time,
    stops: &[&'bn str],
    cost_fn: CF,
) -> Option<Path<'bn, C>>
where
    C: Cost,
    CF: Fn(&Edge) -> C,
{
    // Based on the implementation provided through MS Teams.
    let instant = Instant::now();
    if is_invalid(start_name, bn) || stops.iter().any(|s| is_invalid(s, bn)) {
        return None;
    }

    let context = SolutionContext::new(bn, start_name, start_time, cost_fn);
    let max_iterations = stops.len().pow(2);
    let improve_threshold = (max_iterations as f64).sqrt().floor() as usize;
    let aspiration_criteria = context.aspiration_criteria(stops);

    let mut tabu_list = VecDeque::new();
    let mut turns_since_improve = 0;

    let mut current_solution = context.initial_solution(stops);
    let mut best_solution = current_solution.clone();

    for iteration in 0..max_iterations {
        if turns_since_improve > improve_threshold {
            break;
        }

        let mut best_neighbour = current_solution.clone();
        let mut tabu_candidate = (0, 0);

        for i in 0..stops.len() {
            for j in i + 1..stops.len() {
                if thread_rng().gen_bool(0.25) {
                    continue;
                }

                let neighbour = current_solution.mutate(&context, i, j);

                if (!tabu_list.contains(&(i, j)) || neighbour.cost() < aspiration_criteria)
                    && neighbour.cost() < best_neighbour.cost()
                {
                    best_neighbour = neighbour.clone();
                    tabu_candidate = (i, j);
                }
            }
        }

        current_solution = best_neighbour.clone();

        if tabu_list.len() >= stops.len() {
            tabu_list.pop_front();
        }
        tabu_list.push_back(tabu_candidate);

        if best_neighbour.cost() < best_solution.cost() {
            best_solution = best_neighbour.clone();
            turns_since_improve = 0;
        } else {
            turns_since_improve += 1;
        }

        if cfg!(debug_assertions) {
            eprintln!(
                "Iteration #{:0>3} cost: {}",
                iteration.to_string().bold(),
                best_solution.cost().to_string().blue().bold()
            );
        }
    }

    Some(Path {
        edges: context.reconstruct_edges(&best_solution),
        cost: best_solution.cost(),
        runtime: instant.elapsed(),
    })
}

pub fn tabu_time<'bn>(
    bn: &'bn BusNetwork,
    start_name: &'bn str,
    start_time: Time,
    stops: &[&'bn str],
) -> Option<Path<'bn, u32>> {
    tabu(bn, start_name, start_time, stops, |edge| edge.time_min())
}

pub fn tabu_buses<'bn>(
    bn: &'bn BusNetwork,
    start_name: &'bn str,
    start_time: Time,
    stops: &[&'bn str],
) -> Option<Path<'bn, u32>> {
    tabu(bn, start_name, start_time, stops, |edge| edge.bus_count())
}

pub fn tabu_dist<'bn>(
    bn: &'bn BusNetwork,
    start_name: &'bn str,
    start_time: Time,
    stops: &[&'bn str],
) -> Option<Path<'bn, f32>> {
    tabu(bn, start_name, start_time, stops, |edge| edge.distance_km())
}

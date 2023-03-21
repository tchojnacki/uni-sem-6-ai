use crate::{
    graph::{edge::Edge, solution::SolutionContext, state::Cost},
    BusNetwork, Path, Time,
};
use std::{collections::VecDeque, time::Instant};

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

        println!(
            "Iteration {:3}: Best solution cost = {}",
            iteration,
            best_solution.cost()
        );
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

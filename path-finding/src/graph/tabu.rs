use crate::{
    graph::{edge::Edge, solution::Solution, state::Cost},
    BusNetwork, Path, Time,
};
use std::collections::VecDeque;

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

    let max_iterations = stops.len().pow(2);
    let improve_thresh = (max_iterations as f64).sqrt().floor() as usize;
    let aspiration_criteria = Solution::aspiration_criteria(stops, bn, start_time, &cost_fn);

    let mut tabu_list = VecDeque::new();
    let mut turns_improved = 0;

    let mut current_solution = Solution::initial(start_name, start_time, stops, bn, &cost_fn);
    let mut best_solution = current_solution.clone();

    for iteration in 0..max_iterations {
        if turns_improved > improve_thresh {
            break;
        }

        let mut best_neighbour = current_solution.clone();
        let mut tabu_candidate = (0, 0);

        for i in 0..stops.len() {
            for j in i + 1..stops.len() {
                let neighbour = current_solution.mutate(i, j, bn, &cost_fn);

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
            turns_improved = 0;
        } else {
            turns_improved += 1;
        }

        println!(
            "Iteration {:3}: Best solution cost = {}",
            iteration,
            best_solution.cost()
        );
    }

    // println!("Best solution: {:?}", best_solution);
    println!("Best solution cost: {}", best_solution.cost());

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

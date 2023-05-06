use game_theory::{
    ai::{linear_hash, AlphaBeta, Heuristic, Strategy, LINEAR_WEIGHT_LEN},
    elo::{elo_update, INITIAL_ELO},
    GameState, Outcome, Player,
};
use rand::{thread_rng, Rng};
use std::{
    cmp::Ordering,
    sync::mpsc::channel,
    thread::{available_parallelism, scope},
    time::{Duration, Instant},
};

const MINIMAX_DEPTH: u32 = 3;
const TIME_PER_GEN: Duration = Duration::from_secs(30);
const GEN_COUNT: usize = 100;

const POPULATION_SIZE: usize = 100;
const ELITISM_SIZE: usize = 20;
const FRESH_SIZE: usize = 10;
const MUTATION_MAGNITUDE: f64 = 0.1;

type Chromosome = [f64; LINEAR_WEIGHT_LEN];

fn random_chromosome() -> Chromosome {
    [0.; LINEAR_WEIGHT_LEN].map(|_| thread_rng().gen_range(-1. ..=1.))
}

fn strategy(chromosome: &Chromosome) -> AlphaBeta {
    AlphaBeta::new(
        Heuristic::LinearEquations(Box::new(*chromosome)),
        MINIMAX_DEPTH,
    )
}

fn print_chromosome(chromosome: &Chromosome) {
    print!("[{:.2}", chromosome[0]);
    for &gene in chromosome.iter().skip(1) {
        print!(",{gene:.2}");
    }
    println!("]");
}

fn main() {
    let mut population = (0..POPULATION_SIZE)
        .map(|_| random_chromosome())
        .collect::<Vec<_>>();

    for generation in 1..=GEN_COUNT {
        println!("Training generation #{generation}...");

        let (tx, rx) = channel();
        let threads = available_parallelism().map(|n| n.get()).unwrap_or(1);
        let start = Instant::now();
        scope(|s| {
            for _ in 0..threads {
                let tx = tx.clone();
                let population = &population;
                s.spawn(move || {
                    while start.elapsed() <= TIME_PER_GEN {
                        let bi = thread_rng().gen_range(0..POPULATION_SIZE);
                        let wi = thread_rng().gen_range(0..POPULATION_SIZE);
                        let mut gs = GameState::random_state_between_inc(3, 5);
                        if gs.outcome().is_some() || bi == wi {
                            continue;
                        }

                        let bs = strategy(&population[bi]);
                        let ws = strategy(&population[wi]);

                        while gs.outcome().is_none() {
                            let position = match gs.turn() {
                                Player::Black => &bs,
                                Player::White => &ws,
                            }
                            .decide(&gs);

                            gs = gs.make_move(position);
                        }

                        tx.send((bi, wi, gs.outcome().unwrap())).unwrap();
                    }
                });
            }
        });
        drop(tx);

        let mut fitness = vec![INITIAL_ELO; POPULATION_SIZE];
        while let Ok((bi, wi, outcome)) = rx.recv() {
            if let Outcome::Winner(winner) = outcome {
                match winner {
                    Player::Black => elo_update(&mut fitness, bi, wi),
                    Player::White => elo_update(&mut fitness, wi, bi),
                }
            }
        }

        let mut scored = population.into_iter().zip(fitness).collect::<Vec<_>>();
        scored.sort_by_key(|(_, f)| -*f);
        (population, fitness) = scored.into_iter().unzip();

        println!("Generation #{generation} best individuals:");
        for i in 0..5 {
            println!(
                "{}. LinEq({:03}), {:>4} MMR",
                i + 1,
                linear_hash(&population[i]),
                fitness[i]
            );
        }
        print_chromosome(&population[0]);

        let fitness_sum = fitness.iter().sum::<i32>() as f64;
        let acc_fitness = fitness
            .iter()
            .scan(0., |acc, &x| {
                *acc += (x as f64) / fitness_sum;
                Some(*acc)
            })
            .collect::<Vec<_>>();

        let mut next_population = Vec::new();
        next_population.extend(population.iter().take(ELITISM_SIZE));
        next_population.extend((0..FRESH_SIZE).map(|_| random_chromosome()));

        while next_population.len() != population.len() {
            let r = thread_rng().gen_range(0. ..=1.);
            let i1 = acc_fitness.iter().position(|&f| f >= r).unwrap();
            let r = thread_rng().gen_range(0. ..=1.);
            let i2 = acc_fitness.iter().position(|&f| f >= r).unwrap();

            let mut new_individual = random_chromosome();
            let crossover_point = thread_rng().gen_range(0. ..=1.);
            for (i, v) in new_individual.iter_mut().enumerate() {
                let i_crossover = thread_rng().gen_range(0. ..=1.);
                *v = match i_crossover.partial_cmp(&crossover_point).unwrap() {
                    Ordering::Less => population[i1][i],
                    Ordering::Equal => *v,
                    Ordering::Greater => population[i2][i],
                };

                let i_mutation = thread_rng().gen_range(-1. ..=1.) * MUTATION_MAGNITUDE;
                *v += i_mutation;
            }

            next_population.push(new_individual);
        }

        population = next_population;
    }
}

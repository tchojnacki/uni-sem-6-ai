use path_finding::{astar_buses, astar_time, BusNetwork, StopHeuristic, Time};
use std::io::{self, Write};

const EXPECTED_SPEED_KMPH: f32 = 10.;
const STOP_HEURISTIC: StopHeuristic = StopHeuristic::Distance {
    changes_per_km: 0.1,
};

fn read_line(prompt: &str) -> String {
    let mut response = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut response).unwrap();
    response.trim().to_string()
}

fn main() {
    let start_name = read_line("Podaj przystanek początkowy A: ");
    let end_name = read_line("Podaj przystanek końcowy B: ");
    let criteria = read_line("Podaj kryterium optymalizacyjne [t/p]: ");
    let start_time = Time::from(read_line("Podaj czas początkowy (np. \"00:00:00\"): ").as_str());

    let bn = BusNetwork::construct("data/connection_graph.csv");

    let path = match criteria.as_str() {
        "t" => astar_time(&bn, &start_name, start_time, &end_name, EXPECTED_SPEED_KMPH),
        "p" => astar_buses(&bn, &start_name, start_time, &end_name, STOP_HEURISTIC),
        _ => panic!("Błędne kryterium optymalizacyjne!"),
    };

    if let Some(path) = path {
        println!("{}", path);
        eprintln!("{}", path.metrics());
    } else {
        panic!("Podano błędne dane!");
    }
}

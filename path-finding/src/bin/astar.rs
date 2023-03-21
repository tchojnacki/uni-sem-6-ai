use path_finding::{astar_buses, astar_time, read_line, BusNetwork, StopHeuristic, Time};

const EXPECTED_SPEED_KMPH: f32 = 10.;
const STOP_HEURISTIC: StopHeuristic = StopHeuristic::Distance {
    changes_per_km: 0.1,
};

fn main() {
    let bn = BusNetwork::construct("data/connection_graph.csv");

    let start_name = read_line("Podaj przystanek początkowy A: ");
    let end_name = read_line("Podaj przystanek końcowy B: ");
    let criteria = read_line("Podaj kryterium optymalizacyjne [t/p]: ");
    let start_time = Time::from(read_line("Podaj czas początkowy (np. \"00:00:00\"): ").as_str());

    let path = match criteria.as_str() {
        "t" => astar_time(&bn, &start_name, start_time, &end_name, EXPECTED_SPEED_KMPH),
        "p" => astar_buses(&bn, &start_name, start_time, &end_name, STOP_HEURISTIC),
        _ => panic!("Błędne kryterium optymalizacyjne!"),
    };

    if let Some(path) = path {
        println!("{}", path);
        eprintln!("{}", path.metrics());
    } else {
        println!("Podano błędne dane!");
    }
}

use path_finding::{
    astar_buses, astar_dist, astar_time, display, read_line, BusNetwork, StopHeuristic, Time,
};

const EXPECTED_SPEED_KMPH: f32 = 10.;
const STOP_HEURISTIC: StopHeuristic = StopHeuristic::Distance {
    changes_per_km: 0.1,
};

fn main() {
    let bn = BusNetwork::construct("data/connection_graph.csv");

    let start_name = read_line("Podaj przystanek początkowy A: ");
    let end_name = read_line("Podaj przystanek końcowy B: ");
    let criteria = read_line("Podaj kryterium optymalizacyjne [t/p/d]: ");
    let start_time = Time::from(read_line("Podaj czas początkowy (np. \"00:00:00\"): ").as_str());

    match criteria.as_str() {
        "t" => display(astar_time(
            &bn,
            &start_name,
            start_time,
            &end_name,
            EXPECTED_SPEED_KMPH,
        )),
        "p" => display(astar_buses(
            &bn,
            &start_name,
            start_time,
            &end_name,
            STOP_HEURISTIC,
        )),
        "d" => display(astar_dist(&bn, &start_name, start_time, &end_name)),
        _ => panic!("Błędne kryterium optymalizacyjne!"),
    }
}

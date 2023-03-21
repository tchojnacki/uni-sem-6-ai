use path_finding::{display, read_line, tabu_buses, tabu_dist, tabu_time, BusNetwork, Time};

fn main() {
    let bn = &BusNetwork::construct("data/connection_graph.csv");

    let start_name = read_line("Podaj przystanek początkowy A: ");
    let stops = read_line("Podaj przystanki do odwiedzenia (oddzielone średnikiem): ");
    let stops = stops.split(';').collect::<Vec<&str>>();
    let criteria = read_line("Podaj kryterium optymalizacyjne [t/p/d]: ");
    let start_time = Time::from(read_line("Podaj czas początkowy (np. \"00:00:00\"): ").as_str());

    match criteria.as_str() {
        "t" => display(tabu_time(bn, &start_name, start_time, &stops)),
        "p" => display(tabu_buses(bn, &start_name, start_time, &stops)),
        "d" => display(tabu_dist(bn, &start_name, start_time, &stops)),
        _ => panic!("Błędne kryterium optymalizacyjne!"),
    }
}

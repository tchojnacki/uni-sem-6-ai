use path_finding::{astar_time, dijkstra_time, BusNetwork, Time};

fn main() {
    let bn = BusNetwork::construct("data/connection_graph.csv");

    let path = dijkstra_time(&bn, "Kątna", Time::new(12, 34), "Lubiatów").unwrap();
    println!("{}", path);
    eprintln!("{}", path.metrics());

    let path = astar_time(
        &bn,
        "PL. GRUNWALDZKI",
        Time::new(22, 00),
        "Bielany Wrocławskie - PKP",
        50.,
    )
    .unwrap();
    println!("{}", path);
    eprintln!("{}", path.metrics());
}

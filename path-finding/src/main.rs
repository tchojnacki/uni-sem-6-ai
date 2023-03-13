use path_finding::{dijkstra_buses, dijkstra_distance, dijkstra_time, BusNetwork, Time};

fn main() {
    let bn = BusNetwork::construct("data/connection_graph.csv");

    println!(
        "{}",
        dijkstra_time(&bn, "Kątna", Time::new(12, 34), "Lubiatów").unwrap()
    );

    println!(
        "{}",
        dijkstra_distance(&bn, "Kątna", Time::new(12, 34), "Lubiatów").unwrap()
    );

    println!(
        "{}",
        dijkstra_buses(&bn, "Kątna", Time::new(12, 34), "Lubiatów").unwrap()
    );
}

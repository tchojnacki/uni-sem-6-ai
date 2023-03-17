use path_finding::{dijkstra_time, BusNetwork, Time};

fn main() {
    let bn = BusNetwork::construct("data/connection_graph.csv");

    println!(
        "{}",
        dijkstra_time(&bn, "Kątna", Time::new(12, 34), "Lubiatów").unwrap()
    );
}

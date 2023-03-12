use path_finding::{BusNetwork, Dijkstra, Time};

fn main() {
    let bn = BusNetwork::construct("data/connection_graph.csv");

    println!(
        "{}",
        bn.dijkstra_time("Kątna", Time::new(12, 34), "Lubiatów")
            .unwrap()
    );

    println!(
        "{}",
        bn.dijkstra_distance("Kątna", Time::new(12, 34), "Lubiatów")
            .unwrap()
    );

    println!(
        "{}",
        bn.dijkstra_changes("Kątna", Time::new(12, 34), "Lubiatów")
            .unwrap()
    );
}

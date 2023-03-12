use path_finding::{BusNetwork, Time};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let bn = BusNetwork::construct("data/connection_graph.csv");
    println!("Parsing: {} ms", start.elapsed().as_millis());
    let path = bn.dijkstra("Kątna", Time::new(12, 34), "Lubiatów").unwrap();
    println!(
        "Found {} min path in {} ms",
        path.cost(),
        path.runtime().as_millis()
    );
}

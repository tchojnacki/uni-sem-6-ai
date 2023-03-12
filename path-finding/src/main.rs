use crate::time::Time;
use bus_network::BusNetwork;
use std::time::Instant;

mod bus_network;
mod file_parser;
mod pos;
mod string_pool;
mod time;
mod vec3;

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

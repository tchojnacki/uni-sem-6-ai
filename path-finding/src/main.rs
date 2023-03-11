use std::time::Instant;

use bus_network::BusNetwork;

mod bus_network;
mod pos;
mod time;
mod vec3;

fn main() {
    let start = Instant::now();
    let bn = BusNetwork::construct("data/connection_graph.csv");
    let parsing = start.elapsed();
    bn.bfs("ZOO", "Gajowa");

    println!("Parsing: {}ms", parsing.as_millis());
    loop {}
}

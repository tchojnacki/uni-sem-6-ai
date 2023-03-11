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
    let parsing = start.elapsed();
    bn.bfs("ZOO", Time::from("23:59:00"), "Gajowa");

    println!("Parsing: {}ms", parsing.as_millis());
}

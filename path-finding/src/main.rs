use bus_network::BusNetwork;

mod bus_network;
mod pos;
mod time;
mod vec3;

fn main() {
    BusNetwork::construct("data/connection_graph.csv");
}

use path_finding::{BusNetwork, Dijkstra, Time};

fn main() {
    let bn = BusNetwork::construct("data/connection_graph.csv");
    let path = bn.dijkstra("Kątna", Time::new(12, 34), "Lubiatów").unwrap();
    println!("{}", path);
}

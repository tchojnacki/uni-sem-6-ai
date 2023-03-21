use path_finding::{tabu_time, BusNetwork, Time};

fn main() {
    let bn = &BusNetwork::construct("data/connection_graph.csv");
    let start_name = "ZOO";
    let start_time = Time::new(12, 00);
    let stops = &[
        "Kochanowskiego",
        "Grudziądzka",
        "Bałtycka",
        "Niedźwiedzia",
        "FAT",
        "Hallera",
        "Kamienna",
        "DWORZEC GŁÓWNY",
        "GALERIA DOMINIKAŃSKA",
        "PL. GRUNWALDZKI",
    ];

    tabu_time(bn, start_name, start_time, stops);
}

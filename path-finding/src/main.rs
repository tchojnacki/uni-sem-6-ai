use path_finding::{astar_buses, BusNetwork, StopHeuristic, Time};

fn main() {
    let bn = BusNetwork::construct("data/connection_graph.csv");

    // let path = dijkstra_time(
    //     &bn,
    //     "PL. GRUNWALDZKI",
    //     Time::new(12, 34),
    //     "Głoska - ul. Średzka",
    // )
    // .unwrap();
    // println!("{}", path);
    // eprintln!("{}", path.metrics());

    // let path = astar_time(
    //     &bn,
    //     "PL. GRUNWALDZKI",
    //     Time::new(12, 34),
    //     "Głoska - ul. Średzka",
    //     5.,
    // )
    // .unwrap();
    // println!("{}", path);
    // eprintln!("{}", path.metrics());

    for sh in [
        StopHeuristic::Disabled,
        StopHeuristic::Distance {
            changes_per_km: 0.1,
        },
        StopHeuristic::StopNodes {
            changes_per_node: 0.001,
        },
    ] {
        let path = astar_buses(
            &bn,
            "PL. GRUNWALDZKI",
            Time::new(12, 34),
            "Głoska - ul. Średzka",
            sh,
        )
        .unwrap();
        println!("{}", path);
        eprintln!("{}", path.metrics());
    }
}

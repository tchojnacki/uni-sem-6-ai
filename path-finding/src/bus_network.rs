use crate::{file_parser::row_iter, pos::Pos, time::Time};
use smol_str::SmolStr;
use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, BinaryHeap, HashMap, VecDeque},
    rc::Rc,
    time::{Duration, Instant},
};

type NodeIndex = usize;

#[derive(Debug, PartialEq)]
struct Stop {
    name: Rc<str>,
    pos: Pos,
}

struct Node {
    stop: Stop,
    time: Time,
    line: Option<SmolStr>,
}

impl Node {
    fn new(name: &Rc<str>, pos: Pos, time: Time, line: Option<SmolStr>) -> Self {
        Node {
            stop: Stop {
                name: name.clone(),
                pos,
            },
            time,
            line,
        }
    }
}

#[derive(Debug)]
pub enum Edge<'a> {
    Wait {
        at_stop: &'a Stop,
        from_time: Time,
        to_time: Time,
    },
    Ride {
        on_line: &'a SmolStr,
        from_stop: &'a Stop,
        from_time: Time,
        to_stop: &'a Stop,
        to_time: Time,
    },
    Enter {
        line: &'a SmolStr,
        at_stop: &'a Stop,
        at_time: Time,
    },
    Leave {
        line: &'a SmolStr,
        at_stop: &'a Stop,
        at_time: Time,
    },
}

impl Edge<'_> {
    fn from<'a>(start: &'a Node, end: &'a Node) -> Edge<'a> {
        match (&start.line, &end.line) {
            (None, None) => {
                assert_eq!(start.stop.name, end.stop.name);
                // TODO: Handle walking between same-named stops
                Edge::Wait {
                    at_stop: &start.stop,
                    from_time: start.time,
                    to_time: end.time,
                }
            }
            (Some(sl), Some(el)) => {
                assert_eq!(sl, el);
                Edge::Ride {
                    on_line: sl,
                    from_stop: &start.stop,
                    from_time: start.time,
                    to_stop: &end.stop,
                    to_time: end.time,
                }
            }
            (None, Some(el)) => {
                assert_eq!(start.stop, end.stop);
                assert_eq!(start.time, end.time);
                Edge::Enter {
                    line: el,
                    at_stop: &start.stop,
                    at_time: start.time,
                }
            }
            (Some(sl), None) => {
                assert_eq!(start.stop, end.stop);
                assert_eq!(start.time, end.time);
                Edge::Leave {
                    line: sl,
                    at_stop: &start.stop,
                    at_time: start.time,
                }
            }
        }
    }
}

pub struct Path<'a> {
    edges: Vec<Edge<'a>>,
    cost: u32,
    runtime: Duration,
}

impl Path<'_> {
    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn cost(&self) -> u32 {
        self.cost
    }

    pub fn runtime(&self) -> Duration {
        self.runtime
    }
}

type AdjList = Vec<Vec<NodeIndex>>;

pub struct BusNetwork {
    adj_list: AdjList,                             // maps NodeIndex to NodeIndex list
    nodes: Vec<Node>,                              // maps NodeIndex to Node
    name_lookup: HashMap<Rc<str>, Vec<NodeIndex>>, // maps node name to time-sorted NodeIndex list
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    cost: u32,
    node_index: NodeIndex,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl BusNetwork {
    pub fn construct(csv_path: &str) -> Self {
        let mut node_cache = HashMap::new();

        let mut adj_list = Vec::new();
        let mut nodes = Vec::new();
        let mut name_lookup = HashMap::<Rc<str>, Vec<NodeIndex>>::new();

        let mut add_node = |nodes: &mut Vec<Node>,
                            adj_list: &mut AdjList,
                            id: String,
                            node: Node| match node_cache.entry(id) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(v) => {
                let i = nodes.len();
                nodes.push(node);
                adj_list.push(Vec::new());
                v.insert(i);
                i
            }
        };

        let add_edge = |adj_list: &mut AdjList, from: NodeIndex, to: NodeIndex| {
            assert_ne!(from, to);
            adj_list[from]
                .binary_search(&to)
                .map_err(|i| adj_list[from].insert(i, to))
                .ok()
        };

        let mut add_name_lookup = |nodes: &Vec<Node>, index: NodeIndex| match name_lookup
            .entry(nodes[index].stop.name.clone())
        {
            Entry::Occupied(mut o) => {
                o.get()
                    .binary_search_by_key(&nodes[index].time, |&i| nodes[i].time)
                    .map_err(|i| o.get_mut().insert(i, index))
                    .ok();
            }
            Entry::Vacant(v) => {
                v.insert(vec![index]);
            }
        };

        for r in row_iter(csv_path) {
            let s_junc = Node::new(&r.s_name, r.s_pos, r.s_time, None);
            let s_node = Node::new(&r.s_name, r.s_pos, r.s_time, Some(r.line.clone()));
            let e_junc = Node::new(&r.e_name, r.e_pos, r.e_time, None);
            let e_node = Node::new(&r.e_name, r.e_pos, r.e_time, Some(r.line));

            let s_node_index = add_node(&mut nodes, &mut adj_list, r.s_node_id, s_node);
            let e_node_index = add_node(&mut nodes, &mut adj_list, r.e_node_id, e_node);
            let s_junc_index = add_node(&mut nodes, &mut adj_list, r.s_junc_id, s_junc);
            let e_junc_index = add_node(&mut nodes, &mut adj_list, r.e_junc_id, e_junc);

            add_edge(&mut adj_list, s_junc_index, s_node_index);
            add_edge(&mut adj_list, s_node_index, e_node_index);
            add_edge(&mut adj_list, e_node_index, e_junc_index);

            add_name_lookup(&nodes, s_junc_index);
            add_name_lookup(&nodes, e_junc_index);
        }

        for values in name_lookup.values() {
            for i in 1..values.len() {
                assert!(nodes[values[i - 1]].time < nodes[values[i]].time);
                add_edge(&mut adj_list, values[i - 1], values[i]);
            }
            if values.len() > 1 {
                add_edge(&mut adj_list, values[values.len() - 1], values[0]);
            }
        }

        assert_eq!(adj_list.len(), nodes.len());

        BusNetwork {
            adj_list,
            nodes,
            name_lookup,
        }
    }

    fn find_node_index(&self, name: &str, time: Time) -> usize {
        let times = &self.name_lookup[name];
        times[match times.binary_search_by_key(&time, |&i| self.nodes[i].time) {
            Ok(i) => i,
            Err(i) => i,
        } % times.len()]
    }

    fn reconstruct_edges<'s>(
        &'s self,
        parents: &HashMap<NodeIndex, NodeIndex>,
        to: NodeIndex,
    ) -> Vec<Edge<'s>> {
        let mut current = to;
        let mut path = VecDeque::from([to]);
        while let Some(&parent) = parents.get(&current) {
            path.push_front(parent);
            current = parent;
        }

        path.make_contiguous()
            .windows(2)
            .map(|x| {
                if let [start, end] = x {
                    Edge::from(&self.nodes[*start], &self.nodes[*end])
                } else {
                    unreachable!();
                }
            })
            .collect::<Vec<_>>()
    }

    pub fn dijkstra(&self, start_name: &str, start_time: Time, end_name: &str) -> Option<Path> {
        let instant = Instant::now();

        let start_index = self.find_node_index(start_name, start_time);

        let mut distances = HashMap::<NodeIndex, u32>::with_capacity(self.nodes.len());
        let mut queue = BinaryHeap::<State>::new();
        let mut parents = HashMap::<NodeIndex, NodeIndex>::with_capacity(self.nodes.len());

        distances.insert(start_index, 0);
        queue.push(State {
            cost: 0,
            node_index: start_index,
        });
        while let Some(State {
            cost: curr_dist,
            node_index: curr_node,
        }) = queue.pop()
        {
            if self.nodes[curr_node].stop.name == end_name.into() {
                let edges = self.reconstruct_edges(&parents, curr_node);
                return Some(Path {
                    edges,
                    cost: curr_dist,
                    runtime: instant.elapsed(),
                });
            }

            if Some(&curr_dist) > distances.get(&curr_node) {
                continue;
            }

            for &neighbour in &self.adj_list[curr_node] {
                let weight = self.nodes[neighbour].time - self.nodes[curr_node].time;

                let new_dist = curr_dist + weight;
                if !distances.contains_key(&neighbour) || new_dist < distances[&neighbour] {
                    distances.insert(neighbour, new_dist);
                    parents.insert(neighbour, curr_node);
                    queue.push(State {
                        cost: new_dist,
                        node_index: neighbour,
                    });
                }
            }
        }

        None
    }
}

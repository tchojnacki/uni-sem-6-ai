use crate::{file_parser::row_iter, pos::Pos, time::Time};
use smol_str::SmolStr;
use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    rc::Rc,
};

type NodeIndex = usize;

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

type AdjList = Vec<Vec<NodeIndex>>;

pub struct BusNetwork {
    adj_list: AdjList,                             // maps NodeIndex to NodeIndex list
    nodes: Vec<Node>,                              // maps NodeIndex to Node
    name_lookup: HashMap<Rc<str>, Vec<NodeIndex>>, // maps node name to time-sorted NodeIndex list
}

impl BusNetwork {
    pub fn construct(csv_path: &str) -> BusNetwork {
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
            add_edge(&mut adj_list, values[values.len() - 1], values[0]);
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

    pub fn bfs(&self, from: &str, time: Time, to: &str) {
        let start_index = self.find_node_index(from, time);
        let mut queue = VecDeque::from([start_index]);
        let mut parent = HashMap::new();

        while let Some(index) = queue.pop_front() {
            let node = &self.nodes[index];

            if node.stop.name == to.into() && node.line.is_none() {
                let mut current = index;
                let mut path = VecDeque::from([current]);
                while current != start_index {
                    current = parent[&current];
                    path.push_front(current);
                }
                for n in path {
                    let node = &self.nodes[n];
                    println!(
                        "{} {} {}",
                        node.stop.name,
                        node.time,
                        node.line.clone().unwrap_or(SmolStr::new("-"))
                    );
                }
                return;
            }

            for &n in &self.adj_list[index] {
                if let Entry::Vacant(v) = parent.entry(n) {
                    v.insert(index);
                    queue.push_back(n);
                }
            }
        }
    }
}

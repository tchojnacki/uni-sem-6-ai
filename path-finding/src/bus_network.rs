use crate::{
    pos::{Pos, PosConverter},
    time::Time,
};
use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

struct RowData {
    line: String,
    s_time: Time,
    e_time: Time,
    s_name: String,
    e_name: String,
    s_pos_id: String,
    s_pos: Pos,
    e_pos_id: String,
    e_pos: Pos,
}

type NodeId = (String, String, Time, Option<String>);

impl RowData {
    fn s_node_id(&self) -> NodeId {
        (
            self.s_name.clone(),
            self.s_pos_id.clone(),
            self.s_time,
            Some(self.line.clone()),
        )
    }

    fn s_node(&self) -> Node {
        Node {
            stop: Stop {
                name: self.s_name.to_string(),
                pos: self.s_pos,
            },
            time: self.s_time,
            line: Some(self.line.to_string()),
        }
    }

    fn e_node_id(&self) -> NodeId {
        (
            self.e_name.clone(),
            self.e_pos_id.clone(),
            self.e_time,
            Some(self.line.clone()),
        )
    }

    fn e_node(&self) -> Node {
        Node {
            stop: Stop {
                name: self.e_name.to_string(),
                pos: self.e_pos,
            },
            time: self.e_time,
            line: Some(self.line.to_string()),
        }
    }

    fn s_junc_id(&self) -> NodeId {
        (
            self.s_name.clone(),
            self.s_pos_id.clone(),
            self.s_time,
            None,
        )
    }

    fn s_junc(&self) -> Node {
        Node {
            line: None,
            ..self.s_node()
        }
    }

    fn e_junc_id(&self) -> NodeId {
        (
            self.e_name.clone(),
            self.e_pos_id.clone(),
            self.e_time,
            None,
        )
    }

    fn e_junc(&self) -> Node {
        Node {
            line: None,
            ..self.e_node()
        }
    }
}

type NodeIndex = usize;

struct Stop {
    name: String,
    pos: Pos,
}

struct Node {
    stop: Stop,
    time: Time,
    line: Option<String>,
}

type AdjList = Vec<Vec<NodeIndex>>;

pub struct BusNetwork {
    adj_list: AdjList,                            // maps NodeIndex to NodeIndex list
    nodes: Vec<Node>,                             // maps NodeIndex to Node
    name_lookup: HashMap<String, Vec<NodeIndex>>, // maps node name to time-sorted NodeIndex list
}

impl BusNetwork {
    pub fn construct(csv_path: &str) -> BusNetwork {
        let pc = PosConverter::initialize();
        let mut node_cache = HashMap::new();

        let mut adj_list = Vec::<Vec<NodeIndex>>::new();
        let mut nodes = Vec::<Node>::new();
        let mut name_lookup = HashMap::<String, Vec<NodeIndex>>::new();

        let mut add_node = |nodes: &mut Vec<Node>,
                            adj_list: &mut AdjList,
                            id: NodeId,
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

        let add_directed = |adj_list: &mut AdjList, from: NodeIndex, to: NodeIndex| match adj_list
            [from]
            .binary_search(&to)
        {
            Ok(_) => (),
            Err(i) => adj_list[from].insert(i, to),
        };

        let mut add_name_lookup = |nodes: &Vec<Node>, index: NodeIndex| match name_lookup
            .entry(nodes[index].stop.name.clone())
        {
            Entry::Occupied(mut o) => {
                match o
                    .get()
                    .binary_search_by_key(&nodes[index].time, |&i| nodes[i].time)
                {
                    Ok(_) => (),
                    Err(i) => o.get_mut().insert(i, index),
                }
            }
            Entry::Vacant(v) => {
                v.insert(vec![index]);
            }
        };

        for row in Self::row_iter(csv_path) {
            let r = Self::parse_row(&pc, &row);

            let s_node_index = add_node(&mut nodes, &mut adj_list, r.s_node_id(), r.s_node());
            let e_node_index = add_node(&mut nodes, &mut adj_list, r.e_node_id(), r.e_node());
            let s_junc_index = add_node(&mut nodes, &mut adj_list, r.s_junc_id(), r.s_junc());
            let e_junc_index = add_node(&mut nodes, &mut adj_list, r.e_junc_id(), r.e_junc());

            add_directed(&mut adj_list, s_junc_index, s_node_index);
            add_directed(&mut adj_list, s_node_index, e_node_index);
            add_directed(&mut adj_list, e_node_index, e_junc_index);

            add_name_lookup(&nodes, s_junc_index);
            add_name_lookup(&nodes, e_junc_index);
        }

        for values in name_lookup.values() {
            for i in 0..values.len() {
                add_directed(&mut adj_list, values[i], values[(i + 1) % values.len()]);
            }
        }

        BusNetwork {
            adj_list,
            nodes,
            name_lookup,
        }
    }

    pub fn bfs(&self, from: &str, to: &str) {
        let start_index = self.name_lookup[from][0];
        let mut queue = VecDeque::from([start_index]);
        let mut parent = HashMap::new();

        while let Some(index) = queue.pop_front() {
            let node = &self.nodes[index];

            if node.stop.name == to && node.line.is_none() {
                let mut current = index;
                let mut path = VecDeque::from([current]);
                while current != start_index {
                    current = parent[&current];
                    path.push_front(current);
                }
                for n in path {
                    let node = &self.nodes[n];
                    println!("{} {} {:?}", node.stop.name, node.time, node.line);
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

    fn row_iter(csv_path: &str) -> impl Iterator<Item = String> {
        let file = File::open(csv_path).expect("File not found!");
        let reader = BufReader::new(file);
        reader.lines().skip(1).flatten() // skip column names
    }

    fn parse_row(pc: &PosConverter, line: &str) -> RowData {
        let columns = line.split(',').skip(3).collect::<Vec<_>>(); // skip indices, company
        RowData {
            line: columns[0].to_string(),
            s_time: Time::from(columns[1]),
            e_time: Time::from(columns[2]),
            s_name: columns[3].to_string(),
            e_name: columns[4].to_string(),
            s_pos_id: format!("{} {}", columns[5], columns[6]),
            s_pos: pc.wgs84_to_pos(columns[5], columns[6]),
            e_pos_id: format!("{} {}", columns[7], columns[8]),
            e_pos: pc.wgs84_to_pos(columns[7], columns[8]),
        }
    }
}

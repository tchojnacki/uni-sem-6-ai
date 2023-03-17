use crate::{
    graph::{
        edge::Edge,
        node::{Node, NodeIndex},
    },
    structs::Stop,
    util::file_parser::row_iter,
    Time,
};
use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    rc::Rc,
};

type AdjList = Vec<Vec<NodeIndex>>;

pub struct BusNetwork {
    adj_list: AdjList,                             // maps NodeIndex to NodeIndex list
    nodes: Vec<Node>,                              // maps NodeIndex to Node
    name_lookup: HashMap<Rc<str>, Vec<NodeIndex>>, // maps node name to time-sorted NodeIndex list
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

    pub(super) fn order(&self) -> usize {
        self.nodes.len()
    }

    pub(super) fn node(&self, index: NodeIndex) -> &Node {
        &self.nodes[index]
    }

    pub(super) fn neighbours(&self, of: NodeIndex) -> impl Iterator<Item = NodeIndex> + '_ {
        self.adj_list[of].iter().copied()
    }

    pub(super) fn find_node_index(&self, name: &str, time: Time) -> usize {
        let times = &self.name_lookup[name];
        times[match times.binary_search_by_key(&time, |&i| self.nodes[i].time) {
            Ok(i) => i,
            Err(i) => i,
        } % times.len()]
    }

    pub(super) fn find_stop(&self, stop_name: &str) -> &Stop {
        &self.nodes[self.name_lookup[stop_name][0]].stop
    }

    pub(super) fn is_valid_stop(&self, index: NodeIndex, end_name: &str) -> bool {
        self.nodes[index].stop.name == end_name.into() && self.nodes[index].line.is_none()
    }

    pub(super) fn stop_nodes(&self, stop_name: &str) -> usize {
        self.name_lookup[stop_name].len()
    }

    pub(super) fn reconstruct_edges<'s>(
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
}

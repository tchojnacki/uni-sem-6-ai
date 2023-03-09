use crate::{
    pos::{Pos, PosConverter},
    time::Time,
};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

type NodeIndex = usize;

struct Stop<'a> {
    name: &'a str,
    pos: Pos,
}

struct Node<'a> {
    stop: Stop<'a>,
    time: Time,
}

struct Arc {
    line: Option<String>,
    target: NodeIndex,
}

struct Edge<'a> {
    from: NodeIndex,
    line: Option<&'a str>,
    to: NodeIndex,
}

pub struct BusNetwork<'a> {
    adj_list: Vec<Vec<Arc>>,                      // maps NodeIndex to Arc list
    nodes: Vec<Node<'a>>,                         // maps NodeIndex to Node
    name_lookup: HashMap<String, Vec<NodeIndex>>, // maps node name to time-sorted NodeIndex list
}

impl<'bn> BusNetwork<'bn> {
    pub fn construct(csv_path: &str) -> BusNetwork<'bn> {
        let pc = PosConverter::initialize();

        for row in Self::row_iter(csv_path) {
            let (s_name, s_pos, s_time, line, e_name, e_pos, e_time) = Self::parse_row(&pc, &row);
            todo!()
        }

        todo!()
    }

    fn row_iter(csv_path: &str) -> impl Iterator<Item = String> {
        let file = File::open(csv_path).expect("File not found!");
        let reader = BufReader::new(file);
        reader.lines().skip(1).flatten() // skip column names
    }

    fn parse_row(pc: &PosConverter, line: &str) -> (String, Pos, Time, String, String, Pos, Time) {
        let columns = line.split(',').skip(3).collect::<Vec<_>>(); // skip indices, company
        let line = columns[0].to_owned();
        let s_time = Time::from(columns[1]);
        let e_time = Time::from(columns[2]);
        let s_name = columns[3].to_owned();
        let e_name = columns[4].to_owned();
        let s_pos = pc.wgs84_to_pos(columns[5], columns[6]);
        let e_pos = pc.wgs84_to_pos(columns[7], columns[8]);

        (s_name, s_pos, s_time, line, e_name, e_pos, e_time)
    }
}

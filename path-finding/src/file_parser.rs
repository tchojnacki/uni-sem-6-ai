use crate::{
    pos::{Pos, PosConverter},
    string_pool::StringPool,
    time::Time,
};
use smol_str::SmolStr;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

/// Holds data stored in a single input file row.
pub struct RowData {
    pub line: SmolStr,
    pub s_time: Time,
    pub e_time: Time,
    pub s_name: Rc<str>,
    pub e_name: Rc<str>,
    pub s_pos: Pos,
    pub e_pos: Pos,
    pub s_node_id: String,
    pub s_junc_id: String,
    pub e_node_id: String,
    pub e_junc_id: String,
}

/// Returns RowData for a given input line.
fn parse_row(line: &str, sp: &mut StringPool, pc: &PosConverter) -> RowData {
    let columns = line.split(',').skip(3).collect::<Vec<_>>(); // skip indices, company
    let s_name = sp.resolve(columns[3]);
    let e_name = sp.resolve(columns[4]);

    RowData {
        line: columns[0].into(),
        s_time: Time::from(columns[1]),
        e_time: Time::from(columns[2]),
        s_name,
        e_name,
        s_pos: pc.wgs84_to_pos(columns[5], columns[6]),
        e_pos: pc.wgs84_to_pos(columns[7], columns[8]),
        s_node_id: format!(
            "{},{},{},{},{}",
            columns[3], columns[5], columns[6], columns[1], columns[0]
        ),
        s_junc_id: format!(
            "{},{},{},{}",
            columns[3], columns[5], columns[6], columns[1]
        ),
        e_node_id: format!(
            "{},{},{},{},{}",
            columns[4], columns[7], columns[8], columns[2], columns[0]
        ),
        e_junc_id: format!(
            "{},{},{},{}",
            columns[4], columns[7], columns[8], columns[2]
        ),
    }
}

/// Returns an iterator over RowDatas in a given file.
pub fn row_iter(csv_path: &str) -> impl Iterator<Item = RowData> {
    let mut sp = StringPool::default();
    let pc = PosConverter::initialize();
    let file = File::open(csv_path).expect("File not found!");
    let reader = BufReader::new(file);
    reader
        .lines()
        .skip(1) // skip column names
        .flatten()
        .map(move |line| parse_row(&line, &mut sp, &pc))
}

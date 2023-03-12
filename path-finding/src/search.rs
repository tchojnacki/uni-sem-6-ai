use std::cmp::Ordering;

use crate::bus_network::NodeIndex;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct State {
    pub cost: u32,
    pub node: NodeIndex,
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

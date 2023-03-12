use super::node::NodeIndex;
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct State {
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

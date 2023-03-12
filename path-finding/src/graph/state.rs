use crate::graph::node::NodeIndex;
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq)]
pub(super) struct State<C> {
    pub cost: C,
    pub node: NodeIndex,
}

impl<C: PartialEq> Eq for State<C> {}

impl<C: PartialOrd> Ord for State<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<C: PartialOrd> PartialOrd for State<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

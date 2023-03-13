use crate::graph::node::NodeIndex;
use std::{cmp::Ordering, fmt::Display, ops::Add};

pub trait Cost:
    Sized + Copy + Default + Add<Output = Self> + PartialEq + PartialOrd + Display
{
}
impl<T> Cost for T where
    T: Sized + Copy + Default + Add<Output = Self> + PartialEq + PartialOrd + Display
{
}

#[derive(Clone, Copy, PartialEq)]
pub(super) struct State<C: Cost> {
    pub cost: C,
    pub node: NodeIndex,
}

impl<C: Cost> Eq for State<C> {}

impl<C: Cost> Ord for State<C> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<C: Cost> PartialOrd for State<C> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

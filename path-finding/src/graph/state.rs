use crate::graph::node::NodeIndex;
use std::{cmp::Ordering, fmt::Display, ops::Add};

/// Generic bounds for costs used in Dijsktra and A*. For a value to be used as a cost it must:
/// - have a default (zero) value (Default)
/// - be able to be increased by adding another cost value (Add<Output = Self>)
/// - be comparable with other costs (PartialEq + PartialOrd)
/// - be cheap to move around (Sized + Copy)
/// - have a string representation to show the result (Display)
pub trait Cost:
    Sized + Copy + Default + Add<Output = Self> + PartialEq + PartialOrd + Display
{
}

// Implement the trait for all types satisfying the bounds.
impl<T> Cost for T where
    T: Sized + Copy + Default + Add<Output = Self> + PartialEq + PartialOrd + Display
{
}

/// State held in the priority queue.
/// Ordered (in reverse!) by the cost value.
/// Explicit Ord and PartialOrd implementations make the BinaryHeap min-heap instead of max-heap.
/// Panics if cost values can't be compared (e.g. NaN).
#[derive(Clone, Copy, PartialEq)]
pub(super) struct State<C: Cost> {
    pub cost: C,
    pub node: NodeIndex,
}

impl<C: Cost> Eq for State<C> {}

// https://doc.rust-lang.org/std/collections/binary_heap/index.html
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

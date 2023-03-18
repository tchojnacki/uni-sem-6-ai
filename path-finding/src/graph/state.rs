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
#[derive(Clone, Copy, PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::State;
    use std::{cmp::Ordering, collections::BinaryHeap};

    #[test]
    fn state_has_reversed_order() {
        assert!(State { cost: 3, node: 0 } > State { cost: 5, node: 1 });
        assert!(State { cost: 10, node: 5 } < State { cost: 0, node: 2 });
        assert_eq!(
            State { cost: 0, node: 0 }.cmp(&State { cost: 0, node: 1 }),
            Ordering::Equal
        );
    }

    #[test]
    fn state_turns_max_heap_into_min_heap() {
        let mut pq = BinaryHeap::new();
        pq.push(State { cost: 2, node: 0 });
        pq.push(State { cost: 3, node: 1 });
        pq.push(State { cost: 1, node: 2 });

        assert_eq!(pq.pop(), Some(State { cost: 1, node: 2 }));
        assert_eq!(pq.pop(), Some(State { cost: 2, node: 0 }));
        assert_eq!(pq.pop(), Some(State { cost: 3, node: 1 }));
    }
}

use super::edge::Edge;
use std::time::Duration;

pub struct Path<'a> {
    pub(super) edges: Vec<Edge<'a>>,
    pub(super) cost: u32,
    pub(super) runtime: Duration,
}

impl Path<'_> {
    pub fn cost(&self) -> u32 {
        self.cost
    }

    pub fn runtime(&self) -> Duration {
        self.runtime
    }
}

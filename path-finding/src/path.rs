use std::time::Duration;

use crate::{
    bus_network::{Node, Stop},
    time::Time,
};
use smol_str::SmolStr;

#[derive(Debug)]
pub enum Edge<'a> {
    Wait {
        at_stop: &'a Stop,
        from_time: Time,
        to_time: Time,
    },
    Ride {
        on_line: &'a SmolStr,
        from_stop: &'a Stop,
        from_time: Time,
        to_stop: &'a Stop,
        to_time: Time,
    },
    Enter {
        line: &'a SmolStr,
        at_stop: &'a Stop,
        at_time: Time,
    },
    Leave {
        line: &'a SmolStr,
        at_stop: &'a Stop,
        at_time: Time,
    },
}

impl Edge<'_> {
    pub fn from<'a>(start: &'a Node, end: &'a Node) -> Edge<'a> {
        match (start.line(), end.line()) {
            (None, None) => {
                assert_eq!(start.stop().name, end.stop().name);
                // TODO: Handle walking between same-named stops
                Edge::Wait {
                    at_stop: start.stop(),
                    from_time: start.time(),
                    to_time: end.time(),
                }
            }
            (Some(sl), Some(el)) => {
                assert_eq!(sl, el);
                Edge::Ride {
                    on_line: sl,
                    from_stop: start.stop(),
                    from_time: start.time(),
                    to_stop: end.stop(),
                    to_time: end.time(),
                }
            }
            (None, Some(el)) => {
                assert_eq!(start.stop(), end.stop());
                assert_eq!(start.time(), end.time());
                Edge::Enter {
                    line: el,
                    at_stop: start.stop(),
                    at_time: start.time(),
                }
            }
            (Some(sl), None) => {
                assert_eq!(start.stop(), end.stop());
                assert_eq!(start.time(), end.time());
                Edge::Leave {
                    line: sl,
                    at_stop: start.stop(),
                    at_time: start.time(),
                }
            }
        }
    }
}

pub struct Path<'a> {
    pub edges: Vec<Edge<'a>>,
    pub cost: u32,
    pub runtime: Duration,
}

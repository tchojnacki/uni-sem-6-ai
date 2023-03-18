use crate::{
    graph::node::Node,
    structs::{Stop, Time},
};
use smol_str::SmolStr;
use std::fmt::{self, Display};

/// Represents a transition between passenger states.
#[derive(Debug)]
pub(super) enum Edge<'a> {
    Wait {
        at_stop_name: &'a str,
        from_time: Time,
        to_time: Time,
        distance_km: f32,
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
    pub(super) fn from<'a>(start: &'a Node, end: &'a Node) -> Edge<'a> {
        match (&start.line, &end.line) {
            (None, None) => {
                assert_eq!(start.stop.name, end.stop.name);
                Edge::Wait {
                    at_stop_name: &start.stop.name,
                    from_time: start.time,
                    to_time: end.time,
                    distance_km: start.stop.pos.distance_km(end.stop.pos),
                }
            }
            (Some(sl), Some(el)) => {
                assert_eq!(sl, el);
                Edge::Ride {
                    on_line: sl,
                    from_stop: &start.stop,
                    from_time: start.time,
                    to_stop: &end.stop,
                    to_time: end.time,
                }
            }
            (None, Some(el)) => {
                assert_eq!(start.stop, end.stop);
                assert_eq!(start.time, end.time);
                Edge::Enter {
                    line: el,
                    at_stop: &start.stop,
                    at_time: start.time,
                }
            }
            (Some(sl), None) => {
                assert_eq!(start.stop, end.stop);
                assert_eq!(start.time, end.time);
                Edge::Leave {
                    line: sl,
                    at_stop: &start.stop,
                    at_time: start.time,
                }
            }
        }
    }

    pub(super) fn distance_km(&self) -> f32 {
        match self {
            Edge::Wait { distance_km, .. } => *distance_km,
            Edge::Ride {
                from_stop, to_stop, ..
            } => from_stop.pos.distance_km(to_stop.pos),
            _ => 0.0,
        }
    }

    pub(super) fn time_min(&self) -> u32 {
        match self {
            Edge::Wait {
                from_time, to_time, ..
            } => *to_time - *from_time,
            Edge::Ride {
                from_time, to_time, ..
            } => *to_time - *from_time,
            _ => 0,
        }
    }

    pub(super) fn bus_count(&self) -> u32 {
        match self {
            Edge::Enter { .. } => 1,
            _ => 0,
        }
    }
}

fn trunc(name: &str) -> String {
    if name.chars().count() > 25 {
        name.chars()
            .take(22)
            .chain("...".chars())
            .collect::<String>()
    } else {
        format!("{:25}", name)
    }
}

impl Display for Edge<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Edge::Wait {
                at_stop_name,
                from_time,
                to_time,
                ..
            } => write!(f, "🕒 --- {} --> {} {}", from_time, to_time, at_stop_name),
            Edge::Ride {
                on_line,
                from_stop,
                from_time,
                to_stop,
                to_time,
            } => write!(
                f,
                "🚌 {:>3} {} {} --> {} {}",
                on_line,
                from_time,
                trunc(&from_stop.name),
                to_time,
                trunc(&to_stop.name)
            ),
            Edge::Enter {
                line,
                at_stop,
                at_time,
            } => write!(f, "🚉 {:>3} {} {}", line, at_time, at_stop.name),
            Edge::Leave {
                line,
                at_stop,
                at_time,
            } => write!(f, "🔚 {:>3} {} {}", line, at_time, at_stop.name),
        }
    }
}

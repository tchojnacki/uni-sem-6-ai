use crate::{
    graph::node::Node,
    structs::{Pos, Stop, Time},
};
use smol_str::SmolStr;
use std::fmt::{self, Display};

#[derive(Debug)]
pub(super) enum Edge<'a> {
    Wait {
        at_stop_name: &'a str,
        from_stop_pos: Pos,
        from_time: Time,
        to_stop_pos: Pos,
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
    pub(super) fn from<'a>(start: &'a Node, end: &'a Node) -> Edge<'a> {
        match (&start.line, &end.line) {
            (None, None) => {
                assert_eq!(start.stop.name, end.stop.name);
                Edge::Wait {
                    at_stop_name: &start.stop.name,
                    from_stop_pos: start.stop.pos,
                    from_time: start.time,
                    to_stop_pos: end.stop.pos,
                    to_time: end.time,
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
            } => writeln!(f, "🕒 --- {} --> {} {}", from_time, to_time, at_stop_name),
            Edge::Ride {
                on_line,
                from_stop,
                from_time,
                to_stop,
                to_time,
            } => writeln!(
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
            } => writeln!(f, "🚉 {:>3} {} {}", line, at_time, at_stop.name),
            Edge::Leave {
                line,
                at_stop,
                at_time,
            } => writeln!(f, "🔚 {:>3} {} {}", line, at_time, at_stop.name),
        }
    }
}

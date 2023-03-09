use std::fmt::{self, Display};

/// Represents a timestamp in a day, specified by hours and minutes.
/// Stores the timestamp as the count of minutes since midnight internally.
#[derive(Clone, Copy)]
pub struct Time(u16);

impl From<&str> for Time {
    /// Converts timestamps formatted as "00:00:00" to Time structs.
    /// Ignores any input after minute count (including seconds).
    fn from(timestamp: &str) -> Self {
        let mut parts = timestamp.split(':');

        let hours = parts
            .next()
            .and_then(|s| s.parse::<u16>().ok())
            .expect("Invalid hour format.");

        let minutes = parts
            .next()
            .and_then(|s| s.parse::<u16>().ok())
            .expect("Invalid minute format.");

        if hours >= 24 || minutes >= 60 {
            panic!("Time must be earlier than 23:59.");
        }

        Time(hours * 60 + minutes)
    }
}

impl Display for Time {
    /// String representation of a Time struct.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.0 / 60;
        let minutes = self.0 % 60;
        write!(f, "{hours:02}:{minutes:02}:00")
    }
}

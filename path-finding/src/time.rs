use std::fmt::{self, Display};

/// Represents a timestamp in a day, specified by hours and minutes.
/// Stores the timestamp as the count of minutes since midnight internally.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

        assert!(hours < 24);
        assert!(minutes < 60);

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

#[cfg(test)]
mod tests {
    use crate::time::Time;

    #[test]
    fn time_is_parsed_and_formatted_correctly() {
        let times = ["00:00:00", "00:01:00", "05:12:00", "21:37:00", "23:59:00"];
        for time in times {
            assert_eq!(time, Time::from(time).to_string());
        }
    }

    #[test]
    #[should_panic]
    fn invalid_time_cant_be_constructed() {
        _ = Time::from("25:12:00");
    }
}

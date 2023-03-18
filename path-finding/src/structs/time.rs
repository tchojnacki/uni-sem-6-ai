use std::{
    fmt::{self, Debug, Display},
    ops::Sub,
};

/// Represents a timestamp in a day, specified by hours and minutes.
/// Stores the timestamp as the count of minutes since midnight internally.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time(u16);

impl Time {
    pub fn new(hours: u16, minutes: u16) -> Self {
        assert!(hours < 24);
        assert!(minutes < 60);
        Time(hours * 60 + minutes)
    }
}

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

        Time::new(hours, minutes)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.0 / 60;
        let minutes = self.0 % 60;
        write!(f, "{hours:02}:{minutes:02}")
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Sub for Time {
    type Output = u32;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.0 >= rhs.0 {
            (self.0 - rhs.0) as u32
        } else {
            (60 * 24 - self.0 + rhs.0) as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::structs::time::Time;

    #[test]
    fn time_is_parsed_and_formatted_correctly() {
        let times = ["00:00:00", "00:01:00", "05:12:00", "21:37:00", "23:59:00"];
        for time in times {
            assert_eq!(&time[..5], Time::from(time).to_string());
        }
    }

    #[test]
    #[should_panic]
    fn invalid_time_cant_be_constructed() {
        _ = Time::from("25:12:00");
    }
}

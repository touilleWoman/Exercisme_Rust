use chrono::{Duration, NaiveTime};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    time: NaiveTime,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            time: NaiveTime::from_hms(0, 0, 0)
                + Duration::hours(hours as i64)
                + Duration::minutes(minutes as i64),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            time: self.time + Duration::minutes(minutes as i64),
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted_time = self.time.format("%H:%M").to_string();

        write!(f, "{}", formatted_time)
    }
}

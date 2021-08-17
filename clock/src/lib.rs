use std::fmt;

use chrono::{Duration, NaiveTime, Timelike};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time = NaiveTime::from_hms(0, 0, 0);
        let (time, _nb) = time.overflowing_add_signed(Duration::minutes(minutes.into()));
        let (time, _nb) = time.overflowing_add_signed(Duration::hours(hours.into()));

        Clock {
            hours: time.hour() as i32,
            minutes: time.minute() as i32,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time = NaiveTime::from_hms(self.hours as u32, self.minutes as u32, 0);
        let (time, _nb) = time.overflowing_add_signed(Duration::minutes(minutes.into()));
        Clock {
            hours: time.hour() as i32,
            minutes: time.minute() as i32,
        }
    }
}

use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", (self.minutes / 60).rem_euclid(24), self.minutes.rem_euclid(60))
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { minutes: (hours * 60 + minutes).rem_euclid(1440) }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let m = (self.minutes + minutes).rem_euclid(1440);
        Self {
            minutes: m,
        }
    }
}

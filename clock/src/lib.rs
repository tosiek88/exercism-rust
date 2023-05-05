use core::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let tmp_minutes = (60 * (24 + (24 + (hours % 24))) + minutes % (24 * 60)) % (24 * 60);

        let clock = Clock {
            hours: (24 + ((tmp_minutes / 60) % 24)) % 24,
            minutes: tmp_minutes % 60,
        };
        clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

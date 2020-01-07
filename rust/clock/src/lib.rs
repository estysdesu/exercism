use std::fmt;

const MIN_IN_HR: i32 = 60;
const HRS_IN_DAY: i32 = 24;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(h_raw: i32, m_raw: i32) -> Self {
        let (h_adj, m) = Clock::rollover_minutes(m_raw);
        let h = Clock::rollover_hours(h_raw + h_adj);
        Self {
            hours: h,
            minutes: m,
        }
    }

    fn rollover_minutes(m_raw: i32) -> (i32, i32) {
        let mut m = m_raw % MIN_IN_HR; // remainder
        let mut h = m / MIN_IN_HR; // floor

        // adjust - minutes to + minutes
        if m < 0 {
            m += MIN_IN_HR;
            h -= 1;
        }
        (h, m)
    }

    fn rollover_hours(h_raw: i32) -> i32 {
        let mut h = h_raw % HRS_IN_DAY;
        // CHECK MORE HERE //
        if h_raw < 0 {
            h = -(h_raw.abs() % HRS_IN_DAY); // cannot use integer division with negative, result is 0
        }
        if h < 0 {
            h = HRS_IN_DAY + h;
        }
        h
    }

    pub fn add_minutes(&mut self, m_raw: i32) -> Self {
        let (h_raw, m) = Clock::rollover_minutes(self.minutes + m_raw);
        self.minutes = m;
        self.hours = Clock::rollover_hours(self.hours + h_raw);
        *self
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

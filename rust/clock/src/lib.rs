use std::fmt;

type Minutes = i32;
type Hours = i32;

const MIN_PER_HR: Minutes = 60;
const HRS_PER_DAY: Hours = 24;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Clock {
    hours: Hours,
    minutes: Minutes,
}

impl Clock {
    pub fn new(h_raw: Hours, m_raw: Minutes) -> Self {
        let (h_adj, m) = Clock::rollover_minutes(m_raw);
        let h = Clock::rollover_hours(h_raw + h_adj);
        Self {
            hours: h,
            minutes: m,
        }
    }

    fn rollover_minutes(m_raw: Minutes) -> (Hours, Minutes) {
        let mut m = m_raw % MIN_PER_HR; // remainder
        let mut h = m_raw / MIN_PER_HR; // floor

        // adjust - minutes to + minutes
        if m < 0 {
            m += MIN_PER_HR;
            h -= 1;
        }
        (h, m)
    }

    fn rollover_hours(h_raw: Hours) -> Hours {
        let mut h = h_raw % HRS_PER_DAY;
        if h < 0 {
            h = HRS_PER_DAY + h;
        }
        h
    }

    pub fn add_minutes(&mut self, m_raw: Minutes) -> Self {
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

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
        let h = (h_raw + h_adj).rem_euclid(HRS_PER_DAY);
        Self {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&mut self, m_raw: Minutes) -> Self {
        let (h_raw, m) = Clock::rollover_minutes(self.minutes + m_raw);
        self.minutes = m;
        self.hours = (self.hours + h_raw).rem_euclid(HRS_PER_DAY);
        *self
    }

    fn rollover_minutes(m_raw: Minutes) -> (Hours, Minutes) {
        let m = m_raw.rem_euclid(MIN_PER_HR);
        let h = m_raw.div_euclid(MIN_PER_HR);
        (h, m)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

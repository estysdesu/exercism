use chrono::{DateTime, Duration, Utc};
use std::ops::Add;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start.add(Duration::seconds(1000000000))
}

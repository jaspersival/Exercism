use std::ops::Add;
use time::PrimitiveDateTime as DateTime;
use time::{Duration, Time};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let billion_seconds = Duration::seconds(1e9 as i64);
    start.add(billion_seconds)
}

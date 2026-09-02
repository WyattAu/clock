use std::time::{SystemTime, UNIX_EPOCH};
use crate::Clock;

/// Real system clock (zero-cost abstraction).
///
/// Derives time from `SystemTime::now()`. Uses `UNIX_EPOCH` as the
/// reference point for all calculations.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_ns(&self) -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time is before Unix epoch")
            .as_nanos() as i64
    }
}

impl SystemClock {
    /// Create a new system clock.
    pub fn new() -> Self {
        Self
    }
}

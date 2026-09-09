use crate::Clock;
use std::time::{SystemTime, UNIX_EPOCH};

/// Real system clock (zero-cost abstraction).
///
/// Derives time from `SystemTime::now()`. Uses `UNIX_EPOCH` as the
/// reference point for all calculations.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now_ns(&self) -> i64 {
        // Fallback to 0 if the system clock is set before the Unix epoch
        // (misconfigured machine); a clock library must not panic.
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as i64
    }
}

impl SystemClock {
    /// Create a new system clock.
    pub fn new() -> Self {
        Self
    }
}

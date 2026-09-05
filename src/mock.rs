use std::sync::atomic::{AtomicI64, Ordering};
use crate::Clock;

/// Mock clock for testing with deterministic time control.
///
/// Uses lock-free atomic operations for thread safety.
/// Time starts at 0 and must be advanced manually.
///
/// # Example
/// ```
/// use chronoshift::{Clock, mock::MockClock};
///
/// let clock = MockClock::new(0);
/// assert_eq!(clock.now_ns(), 0);
///
/// clock.advance_ms(1000);
/// assert_eq!(clock.now_ms(), 1000);
/// ```
#[derive(Debug)]
pub struct MockClock {
    time_ns: AtomicI64,
}

impl MockClock {
    /// Create a new mock clock starting at the given time (nanoseconds).
    pub fn new(start_ns: i64) -> Self {
        Self {
            time_ns: AtomicI64::new(start_ns),
        }
    }

    /// Advance the clock by the given number of nanoseconds.
    pub fn advance_ns(&self, nanos: i64) {
        self.time_ns.fetch_add(nanos, Ordering::SeqCst);
    }

    /// Advance the clock by the given number of milliseconds.
    pub fn advance_ms(&self, millis: i64) {
        self.advance_ns(millis * 1_000_000);
    }

    /// Advance the clock by the given number of seconds.
    pub fn advance_secs(&self, secs: i64) {
        self.advance_ns(secs * 1_000_000_000);
    }

    /// Set the clock to a specific time (nanoseconds).
    pub fn set_ns(&self, nanos: i64) {
        self.time_ns.store(nanos, Ordering::SeqCst);
    }

    /// Set the clock to a specific time (milliseconds).
    pub fn set_ms(&self, millis: i64) {
        self.set_ns(millis * 1_000_000);
    }

    /// Get the current time in nanoseconds (alias for now_ns).
    pub fn get_ns(&self) -> i64 {
        self.time_ns.load(Ordering::SeqCst)
    }
}

impl Default for MockClock {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Clone for MockClock {
    fn clone(&self) -> Self {
        Self::new(self.time_ns.load(Ordering::SeqCst))
    }
}

impl Clock for MockClock {
    fn now_ns(&self) -> i64 {
        self.time_ns.load(Ordering::SeqCst)
    }
}

//! Mock clock for testing with deterministic time control.
//!
//! Uses lock-free atomic operations for thread safety on targets with
//! 64-bit atomics; targets without them (e.g. thumbv7em) fall back to a
//! critical-section-backed cell (the critical-section implementation is
//! provided by the final binary, per critical-section convention).
//! Time starts at 0 and must be advanced manually.
//!
//! # Example
//! ```
//! use chronoshift::{Clock, mock::MockClock};
//!
//! let clock = MockClock::new(0);
//! assert_eq!(clock.now_ns(), 0);
//!
//! clock.advance_ms(1000);
//! assert_eq!(clock.now_ms(), 1000);
//! ```

use crate::Clock;

#[cfg(target_has_atomic = "64")]
mod imp {
    use core::sync::atomic::{AtomicI64, Ordering};

    /// Internal time cell: native 64-bit atomics.
    #[derive(Debug)]
    pub struct TimeCell(AtomicI64);

    impl TimeCell {
        pub const fn new(start_ns: i64) -> Self {
            Self(AtomicI64::new(start_ns))
        }
        pub fn fetch_add(&self, nanos: i64) {
            self.0.fetch_add(nanos, Ordering::SeqCst);
        }
        pub fn store(&self, nanos: i64) {
            self.0.store(nanos, Ordering::SeqCst);
        }
        pub fn load(&self) -> i64 {
            self.0.load(Ordering::SeqCst)
        }
    }
}

#[cfg(not(target_has_atomic = "64"))]
mod imp {
    use core::cell::Cell;
    use critical_section::Mutex;

    /// Internal time cell: critical-section fallback for targets without
    /// 64-bit atomics.
    #[derive(Debug)]
    pub struct TimeCell(Mutex<Cell<i64>>);

    impl TimeCell {
        pub const fn new(start_ns: i64) -> Self {
            Self(Mutex::new(Cell::new(start_ns)))
        }
        pub fn fetch_add(&self, nanos: i64) {
            critical_section::with(|cs| {
                let t = self.0.borrow(cs);
                t.set(t.get().wrapping_add(nanos));
            });
        }
        pub fn store(&self, nanos: i64) {
            critical_section::with(|cs| self.0.borrow(cs).set(nanos));
        }
        pub fn load(&self) -> i64 {
            critical_section::with(|cs| self.0.borrow(cs).get())
        }
    }
}

/// Mock clock for testing with deterministic time control.
#[derive(Debug)]
pub struct MockClock {
    time_ns: imp::TimeCell,
}

impl MockClock {
    /// Create a new mock clock starting at the given time (nanoseconds).
    pub fn new(start_ns: i64) -> Self {
        Self {
            time_ns: imp::TimeCell::new(start_ns),
        }
    }

    /// Advance the clock by the given number of nanoseconds.
    pub fn advance_ns(&self, nanos: i64) {
        self.time_ns.fetch_add(nanos);
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
        self.time_ns.store(nanos);
    }

    /// Set the clock to a specific time (milliseconds).
    pub fn set_ms(&self, millis: i64) {
        self.set_ns(millis * 1_000_000);
    }

    /// Get the current time in nanoseconds (alias for now_ns).
    pub fn get_ns(&self) -> i64 {
        self.time_ns.load()
    }
}

impl Default for MockClock {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Clone for MockClock {
    fn clone(&self) -> Self {
        Self::new(self.time_ns.load())
    }
}

impl Clock for MockClock {
    fn now_ns(&self) -> i64 {
        self.time_ns.load()
    }
}

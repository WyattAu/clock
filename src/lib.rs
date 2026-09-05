#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Trait-based time abstraction with a mock clock for testing.
//!
//! Provides a `Clock` trait with injectable time, a `SystemClock` for
//! production, and a `MockClock` for testing. Enables deterministic
//! time-dependent tests without external clock manipulation.

/// Real system clock implementation.
pub mod real;
/// Mock clock for testing.
pub mod mock;
/// WASM-compatible clock using js_sys::Date.
/// WASM clock backend — only available on `wasm32` targets (js-sys calls
/// panic on native targets, so the module is target-gated, not just
/// feature-gated).
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub mod wasm;

/// Clock trait for injectable time.
///
/// All times are in nanoseconds since Unix epoch for maximum granularity.
/// Derived methods provide milliseconds, microseconds, and chrono conversions.
pub trait Clock: Send + Sync {
    /// Current time in nanoseconds since Unix epoch.
    fn now_ns(&self) -> i64;

    /// Current time in milliseconds since Unix epoch (derived).
    fn now_ms(&self) -> i64 {
        self.now_ns() / 1_000_000
    }

    /// Current time in microseconds since Unix epoch (derived).
    fn now_us(&self) -> i64 {
        self.now_ns() / 1_000
    }

    /// Duration in nanoseconds since the given timestamp.
    fn elapsed_ns(&self, since: i64) -> i64 {
        self.now_ns() - since
    }

    /// Duration in milliseconds since the given timestamp.
    fn elapsed_ms(&self, since: i64) -> i64 {
        self.elapsed_ns(since) / 1_000_000
    }
}

/// Clock trait with chrono support.
#[cfg(feature = "chrono")]
pub trait ClockExt: Clock {
    /// Current time as chrono DateTime<Utc>.
    fn now_chrono(&self) -> chrono::DateTime<chrono::Utc> {
        let ns = self.now_ns();
        let secs = ns / 1_000_000_000;
        let nanos = (ns % 1_000_000_000) as u32;
        chrono::DateTime::from_timestamp(secs, nanos)
            .unwrap_or_default()
    }
}

#[cfg(feature = "chrono")]
impl<T: Clock> ClockExt for T {}

/// Get a default system clock.
pub fn system_clock() -> impl Clock {
    real::SystemClock
}

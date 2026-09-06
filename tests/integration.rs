use chronoshift::{mock::MockClock, real::SystemClock, Clock};

#[test]
fn system_clock_returns_valid_time() {
    let clock = SystemClock;
    let now = clock.now_ns();
    assert!(now > 0);
    assert!(now > 1_000_000_000_000_000_000); // After year 2001
}

#[test]
fn mock_clock_starts_at_zero() {
    let clock = MockClock::new(0);
    assert_eq!(clock.now_ns(), 0);
}

#[test]
fn mock_clock_advance() {
    let clock = MockClock::new(0);
    clock.advance_ms(1000);
    assert_eq!(clock.now_ms(), 1000);
    assert_eq!(clock.now_ns(), 1_000_000_000);
}

#[test]
fn mock_clock_set() {
    let clock = MockClock::new(0);
    clock.set_ms(5000);
    assert_eq!(clock.now_ms(), 5000);
}

#[test]
fn elapsed_time_calculation() {
    let clock = MockClock::new(1_000_000); // 1ms in nanoseconds
    clock.advance_ms(500);
    assert_eq!(clock.elapsed_ns(1_000_000), 500_000_000);
    assert_eq!(clock.elapsed_ms(1_000_000), 500);
}

#[test]
fn mock_clock_clone() {
    let clock = MockClock::new(1_000_000_000); // 1 second = 1000 ms
    let clock2 = clock.clone();
    clock.advance_ms(500);
    // Clone captured time at clone point
    assert_eq!(clock2.now_ms(), 1000);
    assert_eq!(clock.now_ms(), 1500);
}

#[cfg(feature = "chrono")]
#[test]
fn chrono_conversion() {
    use chrono::Datelike;
    use chronoshift::ClockExt;
    let clock = MockClock::new(1_000_000_000_000_000_000); // ~2001
    let dt = clock.now_chrono();
    assert!(dt.year() >= 2001);
}

// --- Mutation-killing tests (cargo-mutants triage) ---

#[test]
fn now_us_divides_by_one_thousand() {
    // Kills lib.rs now_us mutants: return 0/1/-1, `/`→`%`, `/`→`*`.
    let clock = MockClock::new(1_500_000);
    assert_eq!(clock.now_us(), 1500);
}

#[test]
fn advance_secs_multiplies_by_one_billion() {
    // Kills mock.rs advance_secs mutants: body→(), `*`→`+`, `*`→`/`.
    let clock = MockClock::new(0);
    clock.advance_secs(2);
    assert_eq!(clock.get_ns(), 2_000_000_000);
}

#[test]
fn get_ns_returns_stored_value() {
    // Kills mock.rs get_ns mutants: return 0/1/-1.
    let clock = MockClock::new(0);
    clock.set_ns(123_456);
    assert_eq!(clock.get_ns(), 123_456);
}

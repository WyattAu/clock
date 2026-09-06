use chronoshift::{mock::MockClock, Clock};
use proptest::prelude::*;

proptest! {
    #[test]
    fn advance_never_decreases_time(millis in 0i64..1_000_000) {
        let clock = MockClock::new(0);
        let before = clock.now_ns();
        clock.advance_ms(millis);
        let after = clock.now_ns();
        prop_assert!(after >= before);
    }

    #[test]
    fn set_always_exact(millis in 0i64..1_000_000_000) {
        let clock = MockClock::new(0);
        clock.set_ms(millis);
        prop_assert_eq!(clock.now_ms(), millis);
    }

    #[test]
    fn elapsed_is_non_negative(since in 0i64..1_000, advance in 0i64..1000) {
        let clock = MockClock::new(since * 1_000_000_000);
        clock.advance_ms(advance);
        prop_assert!(clock.elapsed_ms(since * 1_000_000_000) >= 0);
    }
}

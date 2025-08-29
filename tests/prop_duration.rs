//! Property-based tests for `Duration` conversions and ordering.

use benchmark::Duration;
use proptest::prelude::*;

proptest! {
    #[test]
    fn conversions_monotonic(n in 0u128..=u128::MAX/2) {
        let d = Duration::from_nanos(n);
        // unit projections are monotonic and within bounds
        prop_assert!(d.as_micros() <= d.as_nanos());
        prop_assert!(d.as_millis() <= d.as_micros());
        // seconds as float within sane range
        let s64 = d.as_secs_f64();
        let s32 = d.as_secs_f32();
        prop_assert!(s64 >= 0.0 && s32 >= 0.0);
    }

    #[test]
    fn ordering_consistent(a in 0u128..=1_000_000_000_000u128, b in 0u128..=1_000_000_000_000u128) {
        let da = Duration::from_nanos(a);
        let db = Duration::from_nanos(b);
        prop_assert_eq!(da.cmp(&db), a.cmp(&b));
    }
}

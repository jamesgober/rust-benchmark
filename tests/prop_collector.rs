//! Property-based tests for Collector invariants.

#![cfg(feature = "collector")]

use benchmark::{Collector, Duration, Measurement};
use proptest::prelude::*;

proptest! {
    #[test]
    fn record_order_independent(durations in proptest::collection::vec(1u128..=1_000_000u128, 1..500)) {
        let c1 = Collector::new();
        for d in durations.iter() {
            c1.record_duration("a", Duration::from_nanos(*d));
        }
        let s1 = c1.stats("a").unwrap();

        // Permute by reversing
        let c2 = Collector::new();
        for d in durations.iter().rev() {
            c2.record_duration("a", Duration::from_nanos(*d));
        }
        let s2 = c2.stats("a").unwrap();

        prop_assert_eq!(s1.count, s2.count);
        prop_assert_eq!(s1.min, s2.min);
        prop_assert_eq!(s1.max, s2.max);
        // Mean equality within tolerance due to integer->float conversions in display
        prop_assert_eq!(s1.total, s2.total);
    }

    #[test]
    fn record_vs_measurement_equivalence(durations in proptest::collection::vec(1u128..=10_000u128, 1..200)) {
        let c = Collector::new();
        for d in durations.iter() {
            let m = Measurement { name: "x", duration: Duration::from_nanos(*d), timestamp: 0 };
            c.record(&m);
        }
        let s = c.stats("x").unwrap();
        prop_assert_eq!(s.count as usize, durations.len());
        prop_assert!(s.min.as_nanos() <= s.max.as_nanos());
    }
}

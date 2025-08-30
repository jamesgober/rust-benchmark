//! Property-based tests for Histogram invariants.

#![cfg(feature = "collector")]

use benchmark::histogram::Histogram;
use proptest::prelude::*;

proptest! {
    // Random sequences of u64 within a wide range; ensure invariants hold.
    #[test]
    fn histogram_invariants(values in proptest::collection::vec(1u64..=500_000u64, 1..200)) {
        let h = Histogram::default();
        for v in &values {
            h.record(*v);
        }
        let count = h.count();
        prop_assert!(count as usize == values.len());

        if count > 0 {
            let min = h.min().unwrap();
            let max = h.max().unwrap();
            let mean = h.mean().unwrap();
            let p50 = h.percentile(0.50).unwrap();
            let p90 = h.percentile(0.90).unwrap();
            let p99 = h.percentile(0.99).unwrap();

            prop_assert!(min <= max);
            prop_assert!(min as f64 <= mean && mean <= max as f64);
            prop_assert!(p50 <= p90 && p90 <= p99);
            prop_assert!(min <= p50 && p99 <= max);
        }
    }

    #[test]
    fn percentiles_monotonic(values in proptest::collection::vec(1u64..=1_000_000u64, 5..100)) {
        let h = Histogram::default();
        for v in &values { h.record(*v); }
        // Check a sequence of increasing percentiles is non-decreasing
        let ps = [0.0, 0.01, 0.05, 0.10, 0.50, 0.90, 0.95, 0.99, 0.999, 1.0];
        let mut last = 0u64;
        for (i, p) in ps.iter().enumerate() {
            let val = h.percentile(*p).unwrap();
            if i > 0 { prop_assert!(val >= last); }
            last = val;
        }
    }
}

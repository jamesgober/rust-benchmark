#![cfg(feature = "std")]
// Focused hardening tests for invariants, edge handling, and boundary conversions.

use benchmark::{histogram::Histogram, Collector, Duration};

#[cfg(feature = "metrics")]
use benchmark::Watch;

#[test]
fn histogram_empty_returns_none() {
    let h = Histogram::default();
    assert_eq!(h.count(), 0);
    assert!(h.min().is_none());
    assert!(h.max().is_none());
    assert!(h.mean().is_none());
    assert!(h.percentile(0.0).is_none());
    assert!(matches!(
        h.percentiles(&[0.0, 0.5, 1.0]).as_slice(),
        [None, None, None]
    ));
}

#[test]
fn histogram_single_value_all_percentiles_equal() {
    let h = Histogram::default();
    h.record(123);
    assert_eq!(h.count(), 1);
    let v = 123u64;
    assert_eq!(h.min(), Some(v));
    assert_eq!(h.max(), Some(v));
    assert_eq!(h.percentile(0.0), Some(v));
    assert_eq!(h.percentile(0.5), Some(v));
    assert_eq!(h.percentile(1.0), Some(v));
    let ps = h.percentiles(&[0.0, 0.25, 0.5, 0.75, 1.0]);
    for p in ps {
        assert_eq!(p, Some(v));
    }
}

#[test]
fn histogram_out_of_range_percentiles_clamp_to_bounds() {
    let h = Histogram::default();
    for v in [1u64, 100, 10_000] {
        h.record(v);
    }
    let min = h.min().unwrap();
    let max = h.max().unwrap();

    // p < 0 behaves like 0.0 (min)
    let p_neg = h.percentile(-0.25);
    // p > 1 behaves like 1.0 (max)
    let p_big = h.percentile(7.0);
    assert_eq!(p_neg, Some(min));
    assert_eq!(p_big, Some(max));

    let ps = h.percentiles(&[-1.0, 0.0, 0.5, 1.0, 2.0]);
    assert_eq!(ps[0], Some(min));
    assert_eq!(ps[1], Some(min));
    assert!(ps[2].unwrap() >= min && ps[2].unwrap() <= max);
    assert_eq!(ps[3], Some(max));
    assert_eq!(ps[4], Some(max));
}

#[test]
fn histogram_handles_large_values_and_duplicates() {
    let h = Histogram::default();
    let large = u64::MAX - 1;
    for _ in 0..10 {
        h.record(large);
    }
    h.record(u64::MAX);
    assert_eq!(h.count(), 11);
    assert_eq!(h.min().unwrap(), large);
    assert_eq!(h.max().unwrap(), u64::MAX);
    let p99 = h.percentile(0.99).unwrap();
    assert!(p99 >= large);
}

#[test]
fn collector_stats_none_for_missing_key_and_counts_zero_duration() {
    let c = Collector::new();
    assert!(c.stats("missing").is_none());

    c.record_duration("zero", Duration::from_nanos(0));
    let s = c.stats("zero").unwrap();
    assert_eq!(s.count, 1);
    assert_eq!(s.min.as_nanos(), 0);
    assert_eq!(s.max.as_nanos(), 0);
}

#[test]
#[cfg(feature = "metrics")]
fn watch_records_extreme_values() {
    let w = Watch::new();
    w.record("x", 0);
    w.record("x", 1);
    w.record("x", u64::MAX);
    let s = &w.snapshot()["x"];
    // Watch clamps inputs to histogram bounds: [1ns, ~1h]
    assert_eq!(s.min, 1);
    assert_eq!(s.max, 3_600_000_000_000);
}

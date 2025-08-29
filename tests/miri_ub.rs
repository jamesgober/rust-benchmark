// Miri-only UB guard tests: keep fast, single-threaded, and deterministic.
#![cfg(all(miri, feature = "std"))]

use benchmark::{histogram::Histogram, Collector, Duration, Measurement};

#[test]
fn miri_duration_arithmetic_sane() {
    let a = Duration::from_nanos(5);
    let b = Duration::from_nanos(7);
    let c = Duration::from_nanos(a.as_nanos() + b.as_nanos());
    assert_eq!(c.as_nanos(), 12);

    let d = Duration::from_nanos(c.as_nanos().saturating_sub(a.as_nanos()));
    assert_eq!(d.as_nanos(), 7);
}

#[test]
fn miri_histogram_basic_percentiles() {
    let h = Histogram::default();
    for v in [1u64, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
        h.record(v);
    }
    assert_eq!(h.count(), 10);

    let min = h.min().unwrap();
    let max = h.max().unwrap();
    let p0 = h.percentile(0.0).unwrap();
    let p50 = h.percentile(0.5).unwrap();
    let p100 = h.percentile(1.0).unwrap();

    assert!(min <= p50 && p50 <= max);
    assert_eq!(p0, min);
    assert_eq!(p100, max);
}

#[test]
fn miri_collector_record_and_stats() {
    let c = Collector::new();
    for n in 1u128..=10 {
        let m = Measurement {
            name: "miri",
            duration: Duration::from_nanos(n),
            timestamp: 0,
        };
        c.record(&m);
    }
    let s = c.stats("miri").expect("stats present");
    assert_eq!(s.count, 10);
    assert_eq!(s.min.as_nanos(), 1);
    assert_eq!(s.max.as_nanos(), 10);
}

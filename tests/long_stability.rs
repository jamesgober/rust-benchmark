//! Long-running stability tests to detect drift, leaks, or deadlocks.

#![cfg(all(feature = "metrics", feature = "std"))]

use benchmark::Watch;
use std::time::{Duration, Instant};

// Long/perf gated: opt-in only.
#[cfg_attr(
    any(not(feature = "perf-tests"), not(feature = "long-tests")),
    ignore = "long/perf gated; enable -F perf-tests -F long-tests and set LONG_TESTS=1"
)]
#[test]
#[cfg_attr(miri, ignore = "long-running timing loops are not suitable for Miri")]
fn long_running_watch_snapshot_stability() {
    if std::env::var("LONG_TESTS").ok().as_deref() != Some("1") {
        eprintln!("skipping; set LONG_TESTS=1 to run");
        return;
    }

    let w = Watch::new();
    let name = "steady";

    let start = Instant::now();
    let deadline = start + Duration::from_secs(60); // 1 minute

    let mut last_count = 0u64;
    while Instant::now() < deadline {
        for _ in 0..10_000 {
            let t0 = Instant::now();
            std::hint::black_box(1 + 1);
            w.record(name, t0.elapsed().as_nanos() as u64);
        }
        let snap = w.snapshot();
        let s = &snap[name];
        assert!(s.count >= last_count, "count must be non-decreasing");
        last_count = s.count;
    }
}

//! Stress tests for high-frequency measurements on Watch/Timer hot paths.

#![cfg(feature = "metrics")]

use benchmark::{Timer, Watch};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

// Perf-gated: requires explicit opt-in to avoid noisy CI by default.
#[cfg_attr(
    not(feature = "perf-tests"),
    ignore = "perf gated; set PERF_TESTS=1 and enable -F perf-tests and -F metrics"
)]
#[cfg_attr(miri, ignore = "perf-heavy threaded test is not suitable for Miri")]
#[test]
fn stress_watch_record_multi_thread() {
    let threads = 8usize;
    let iters = 200_000usize; // 8 * 200k = 1.6M records
    let w = Arc::new(Watch::new());

    thread::scope(|s| {
        for _ in 0..threads {
            let w = w.clone();
            s.spawn(move || {
                for i in 0..iters {
                    // Alternate between direct and instant-based recording
                    if i % 2 == 0 {
                        let ns = (i as u64 % 1_000) + 1;
                        w.record("hot", ns);
                    } else {
                        let start = Instant::now();
                        std::hint::black_box(i);
                        w.record_instant("hot", start);
                    }
                }
            });
        }
    });

    let snap = w.snapshot();
    let s = &snap["hot"];
    assert_eq!(s.count as usize, threads * iters);
}

#[cfg_attr(
    not(feature = "perf-tests"),
    ignore = "perf gated; set PERF_TESTS=1 and enable -F perf-tests and -F metrics"
)]
#[cfg_attr(miri, ignore = "perf-heavy threaded test is not suitable for Miri")]
#[test]
fn stress_timer_drop_records() {
    let w = Watch::new();
    let total = 200_000usize;
    for _ in 0..total {
        let _t = Timer::new(w.clone(), "tick");
        std::hint::black_box(1 + 1);
        // drop at end of scope records
    }
    let s = &w.snapshot()["tick"];
    assert_eq!(s.count as usize, total);
}

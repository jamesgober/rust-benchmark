//! Cross-platform tests for std::time::Instant behavior.
//! These focus on monotonicity and basic API guarantees that are portable.

use std::time::{Duration, Instant, SystemTime};

#[test]
fn instant_is_monotonic_non_decreasing() {
    // Ensure successive reads are non-decreasing.
    // We allow equality because many platforms have coarse timer granularity.
    let a = Instant::now();
    let b = Instant::now();
    assert!(b >= a, "Instant went backwards: {a:?} -> {b:?}");
}

#[test]
fn instant_eventually_advances_within_reasonable_iters() {
    // On very coarse timers, multiple reads can be equal. We spin for a bounded
    // number of iterations to observe an advancement.
    // Keep runtime minimal; bail out once we see progress.
    let start = Instant::now();
    let mut last = start;
    let mut advanced = false;

    // Up to 1e6 iterations, but usually breaks much earlier.
    for _ in 0..1_000_000 {
        let now = Instant::now();
        if now > last {
            advanced = true;
            break;
        }
        last = now;
    }

    assert!(advanced, "Instant did not advance in 1e6 tight iterations; timer resolution extremely coarse or virtualized environment is too noisy");
}

#[test]
fn duration_arithmetic_is_safe_and_consistent() {
    // Basic checked ops should not panic and behave predictably.
    let d1 = Duration::from_nanos(1);
    let d2 = Duration::from_millis(2);

    assert_eq!(d1 + d2, Duration::from_millis(2) + Duration::from_nanos(1));
    assert!(d2 > d1);

    // Saturating add behavior via checked_add
    let max = Duration::from_secs(u64::MAX);
    assert!(max.checked_add(Duration::from_secs(1)).is_none());
}

#[test]
fn instant_checked_add_sub_apis() {
    let t0 = Instant::now();
    let ten_ms = Duration::from_millis(10);

    if let Some(t1) = t0.checked_add(ten_ms) {
        assert!(t1 >= t0);
        if let Some(t2) = t1.checked_sub(ten_ms) {
            assert!(t2 <= t1);
        }
    }
}

#[test]
#[cfg_attr(miri, ignore)]
fn system_time_vs_instant_semantics_documented() {
    // We cannot reliably force a system time change in CI. This test simply asserts the
    // intended semantic difference: Instant is for durations; SystemTime is wall-clock.
    let _wall = SystemTime::now();
    let _mono = Instant::now();
    // If this compiles and runs, semantics are available. No runtime assertion here.
}

// Optional perf-oriented micro test, gated and ignored by default. Intended to
// be exercised by scheduled perf workflow with PERF_TESTS=1 and -F perf-tests.
#[cfg(feature = "perf-tests")]
mod perf {
    use super::*;

    fn perf_enabled() -> bool {
        std::env::var("PERF_TESTS").ok().as_deref() == Some("1")
    }

    #[test]
    #[ignore]
    fn instant_now_throughput_smoke() {
        if !perf_enabled() {
            eprintln!("PERF_TESTS=1 not set; skipping perf smoke test");
            return;
        }
        // Take a large number of Instant::now() calls as a smoke test; we do not
        // assert a specific threshold due to host variance, only that it completes
        // within a reasonable time window enforced by CI timeout.
        let iters: u64 = 5_000_000;
        let start = Instant::now();
        let mut last = start;
        let mut progressed = 0u64;
        for _ in 0..iters {
            let now = Instant::now();
            if now > last {
                progressed += 1;
            }
            last = now;
        }
        // Ensure at least some progress was observed to avoid completely static timers.
        assert!(
            progressed > 0,
            "no observable timer progress across iterations"
        );
        let elapsed = start.elapsed();
        eprintln!("instant_now_throughput_smoke: iters={iters} elapsed={elapsed:?}");
    }
}

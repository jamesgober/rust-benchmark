#![cfg(feature = "std")]

// Manual observation test for system time changes vs Instant monotonicity.
// This test is intentionally ignored and additionally gated by PERF_TESTS=1
// to avoid accidental execution in CI. Run locally only when you intend to
// adjust the system clock (or let NTP step) while it runs.
//
// Usage:
//   PERF_TESTS=1 cargo test -F perf-tests --tests system_time_change -- --ignored --nocapture
//
use std::env;
use std::thread;
use std::time::{Duration, Instant, SystemTime};

#[test]
#[ignore]
fn observe_system_time_change_vs_instant() {
    // Extra guard: only run when explicitly requested.
    if env::var("PERF_TESTS").ok().as_deref() != Some("1") {
        eprintln!("PERF_TESTS!=1; skipping manual observation test");
        return;
    }

    println!("Manual observation: system time changes vs Instant monotonicity");
    println!("Instructions: while this test runs, manually adjust the system clock forward/backward or let NTP step.");
    println!("Observe that SystemTime may jump while Instant remains monotonic.");

    let start_instant = Instant::now();
    let start_system = SystemTime::now();

    let mut last_instant = start_instant;
    let mut last_system = start_system;

    for i in 0..30 {
        thread::sleep(Duration::from_secs(1));

        let now_instant = Instant::now();
        let now_system = SystemTime::now();

        let di = now_instant.duration_since(last_instant);
        let ds = now_system
            .duration_since(last_system)
            .map(|d| d.as_secs_f64())
            .map_err(|e| e.duration().as_secs_f64());

        let total_i = now_instant.duration_since(start_instant);
        let total_s = now_system
            .duration_since(start_system)
            .map(|d| d.as_secs_f64())
            .map_err(|e| -e.duration().as_secs_f64());

        match ds {
            Ok(s_step) => println!("[{i:02}] Instant +{:.3}s (total {:.3}s) | SystemTime +{s_step:.3}s (total {:?}s)", di.as_secs_f64(), total_i.as_secs_f64(), total_s),
            Err(s_back) => println!("[{i:02}] Instant +{:.3}s (total {:.3}s) | SystemTime -{s_back:.3}s (total {:?}s) [backward jump]", di.as_secs_f64(), total_i.as_secs_f64(), total_s),
        }

        // Monotonic assertion for Instant (should always hold)
        assert!(now_instant >= last_instant, "Instant went backwards!");
        last_instant = now_instant;
        last_system = now_system;
    }
}

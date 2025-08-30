// Demonstrate zero-overhead configuration.
// With default features (std + benchmark), timings are real.
// With --no-default-features (disables `benchmark`), timings are ZERO.
//   cargo run --example zero_overhead                 # real timings
//   cargo run --example zero_overhead --no-default-features -F std  # zero timings

use benchmark::{measure, time, Duration};

fn main() {
    println!("cfg benchmark enabled: {}", cfg!(feature = "benchmark"));

    let (val, d1): (u64, Duration) = time!((0..1_000_000u64).sum());
    println!("time!: value={val}, duration={} ({} ns)", d1, d1.as_nanos());

    let (val2, d2) = measure(|| (0..1_000_000u64).sum::<u64>());
    println!(
        "measure: value={val2}, duration={} ({} ns)",
        d2,
        d2.as_nanos()
    );

    if !cfg!(feature = "benchmark") {
        assert_eq!(
            d1.as_nanos(),
            0,
            "zero-overhead path returns zero durations"
        );
        assert_eq!(
            d2.as_nanos(),
            0,
            "zero-overhead path returns zero durations"
        );
        println!("zero-overhead verified: durations are zero when benchmark feature is disabled");
    }
}

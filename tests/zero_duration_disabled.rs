#![cfg(not(feature = "enabled"))]

// Integration test asserting zero-duration behavior end-to-end
// when the 'enabled' feature is disabled.

use benchmark::{measure, measure_named, time, time_named, Duration};

#[test]
fn measure_returns_zero_duration_when_disabled() {
    let (out, d) = measure(|| 1 + 1);
    assert_eq!(out, 2);
    assert_eq!(d.as_nanos(), 0, "Duration should be zero when disabled");
}

#[test]
fn time_macro_returns_zero_duration_when_disabled() {
    let (out, d) = time!(3 * 3);
    assert_eq!(out, 9);
    assert_eq!(d.as_nanos(), 0);
}

#[test]
fn measure_named_returns_zero_duration_and_timestamp_when_disabled() {
    let (out, m) = measure_named("op", || 7);
    assert_eq!(out, 7);
    assert_eq!(m.name, "op");
    assert_eq!(m.duration, Duration::ZERO);
    assert_eq!(m.timestamp, 0);
}

#[test]
fn time_named_returns_zero_duration_when_disabled() {
    let (out, m) = time_named!("fast", 5);
    assert_eq!(out, 5);
    assert_eq!(m.name, "fast");
    assert_eq!(m.duration.as_nanos(), 0);
}

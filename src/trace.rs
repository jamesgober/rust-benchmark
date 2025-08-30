//! Internal trace hooks for debugging overhead.
//! Compiles to no-ops unless the `trace` feature is enabled.
#![cfg_attr(all(feature = "trace", not(feature = "metrics")), allow(dead_code))]

#[inline]
#[cfg(feature = "trace")]
pub(crate) fn record_event(name: &str, duration_ns: u64) {
    // Lightweight hook: use eprintln! to avoid external deps.
    // Users can redirect stderr if desired.
    eprintln!("benchmark::trace name={name} ns={duration_ns}");
}

#[inline]
#[cfg(not(feature = "trace"))]
pub(crate) fn record_event(_name: &str, _duration_ns: u64) {
    // Compiles to nothing in release builds.
}

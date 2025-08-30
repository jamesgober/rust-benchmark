//! Internal histogram backend trait abstraction.
//!
//! This allows us to implement multiple histogram backends (fast default,
//! high-precision variants, HDR-backed) while keeping the public API stable
//! via `histogram::Histogram`.
//!
//! The trait is crate-visible and implemented for the current
//! `histogram::Histogram` type for now. Backend selection wiring can evolve
//! without breaking public API.

/// Trait representing the histogram functionality used by `Watch` and other consumers.
///
/// This is public because `Watch` and `WatchBuilder` are public generic aliases
/// bound by this trait, and Rust requires public items' bounds to be public.
pub trait HistBackend {
    fn new() -> Self
    where
        Self: Sized;

    fn record(&self, value_ns: u64);
    fn record_duration(&self, duration: core::time::Duration);

    fn min(&self) -> Option<u64>;
    fn max(&self) -> Option<u64>;
    fn mean(&self) -> Option<f64>;
    fn count(&self) -> u64;
    fn is_empty(&self) -> bool;

    fn percentile(&self, p: f64) -> Option<u64>;
    fn median(&self) -> Option<u64>;
    fn median_duration(&self) -> Option<core::time::Duration>;
    fn percentile_duration(&self, p: f64) -> Option<core::time::Duration>;
    fn percentiles(&self, ps: &[f64]) -> Vec<Option<u64>>;

    fn reset(&self);
}

// Implement for the default fast backend
impl HistBackend for crate::histogram::FastHistogram {
    #[inline]
    fn new() -> Self {
        <crate::histogram::FastHistogram>::new()
    }

    #[inline]
    fn record(&self, value_ns: u64) {
        crate::histogram::FastHistogram::record(self, value_ns);
    }

    #[inline]
    fn record_duration(&self, duration: core::time::Duration) {
        crate::histogram::FastHistogram::record_duration(self, duration);
    }

    #[inline]
    fn min(&self) -> Option<u64> {
        crate::histogram::FastHistogram::min(self)
    }

    #[inline]
    fn max(&self) -> Option<u64> {
        crate::histogram::FastHistogram::max(self)
    }

    #[inline]
    fn mean(&self) -> Option<f64> {
        crate::histogram::FastHistogram::mean(self)
    }

    #[inline]
    fn count(&self) -> u64 {
        crate::histogram::FastHistogram::count(self)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        crate::histogram::FastHistogram::is_empty(self)
    }

    #[inline]
    fn percentile(&self, p: f64) -> Option<u64> {
        crate::histogram::FastHistogram::percentile(self, p)
    }

    #[inline]
    fn median(&self) -> Option<u64> {
        crate::histogram::FastHistogram::median(self)
    }

    #[inline]
    fn median_duration(&self) -> Option<core::time::Duration> {
        crate::histogram::FastHistogram::median_duration(self)
    }

    #[inline]
    fn percentile_duration(&self, p: f64) -> Option<core::time::Duration> {
        crate::histogram::FastHistogram::percentile_duration(self, p)
    }

    #[inline]
    fn percentiles(&self, ps: &[f64]) -> Vec<Option<u64>> {
        crate::histogram::FastHistogram::percentiles(self, ps)
    }

    #[inline]
    fn reset(&self) {
        crate::histogram::FastHistogram::reset(self);
    }
}

// Implement for the HDR backend when enabled
#[cfg(feature = "hdr")]
impl HistBackend for crate::hist_hdr::Histogram {
    #[inline]
    fn new() -> Self {
        <crate::hist_hdr::Histogram>::new()
    }

    #[inline]
    fn record(&self, value_ns: u64) {
        crate::hist_hdr::Histogram::record(self, value_ns);
    }

    #[inline]
    fn record_duration(&self, duration: core::time::Duration) {
        crate::hist_hdr::Histogram::record_duration(self, duration);
    }

    #[inline]
    fn min(&self) -> Option<u64> {
        crate::hist_hdr::Histogram::min(self)
    }

    #[inline]
    fn max(&self) -> Option<u64> {
        crate::hist_hdr::Histogram::max(self)
    }

    #[inline]
    fn mean(&self) -> Option<f64> {
        crate::hist_hdr::Histogram::mean(self)
    }

    #[inline]
    fn count(&self) -> u64 {
        crate::hist_hdr::Histogram::count(self)
    }

    #[inline]
    fn is_empty(&self) -> bool {
        crate::hist_hdr::Histogram::is_empty(self)
    }

    #[inline]
    fn percentile(&self, p: f64) -> Option<u64> {
        crate::hist_hdr::Histogram::percentile(self, p)
    }

    #[inline]
    fn median(&self) -> Option<u64> {
        crate::hist_hdr::Histogram::median(self)
    }

    #[inline]
    fn median_duration(&self) -> Option<core::time::Duration> {
        crate::hist_hdr::Histogram::median_duration(self)
    }

    #[inline]
    fn percentile_duration(&self, p: f64) -> Option<core::time::Duration> {
        crate::hist_hdr::Histogram::percentile_duration(self, p)
    }

    #[inline]
    fn percentiles(&self, ps: &[f64]) -> Vec<Option<u64>> {
        crate::hist_hdr::Histogram::percentiles(self, ps)
    }

    #[inline]
    fn reset(&self) {
        crate::hist_hdr::Histogram::reset(self);
    }
}

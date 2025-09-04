#![cfg(feature = "hdr")]
//! HDR-backed histogram implementation used when the `hdr` feature is enabled.
//!
//! This module provides a `Histogram` that wraps `hdrhistogram::Histogram<u64>`
//! with a thread-safe `RwLock` and mirrors the public API of the fast default
//! histogram for seamless swapping.

use std::sync::RwLock;

/// HDR-backed histogram adapter.
///
/// Thread-safe via `RwLock`, API-compatible with `histogram::Histogram` used
/// internally by `Watch`.
#[derive(Debug)]
pub struct Histogram {
    inner: RwLock<hdrhistogram::Histogram<u64>>, // values are nanoseconds
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new()
    }
}

impl Histogram {
    #[inline]
    /// Creates a new HDR-backed histogram with 1ns..~1h bounds and 3 sigfigs.
    pub fn new() -> Self {
        // 1ns .. ~1h, 3 significant figures by default to match Watch defaults.
        let h = hdrhistogram::Histogram::new_with_bounds(1, 3_600_000_000_000u64, 3)
            .unwrap_or_else(|e| {
                // Bounds are compile-time constants and valid. If construction fails,
                // avoid panicking in release: log via debug assertion and fall back
                // to a histogram with default dynamic max using the same sigfigs.
                debug_assert!(false, "HDR bounds init failed: {e}");
                hdrhistogram::Histogram::new(3).unwrap_or_else(|_| {
                    hdrhistogram::Histogram::new_with_max(3_600_000_000_000u64, 3).unwrap()
                })
            });
        Self {
            inner: RwLock::new(h),
        }
    }

    #[inline]
    /// Record a value in nanoseconds.
    pub fn record(&self, value_ns: u64) {
        // Saturate to configured bounds [1ns, 1h]
        let v = value_ns.clamp(1, 3_600_000_000_000u64);
        if let Ok(mut h) = self.inner.write() {
            let _ = h.record(v);
        }
    }

    #[inline]
    /// Record a `Duration` by converting to nanoseconds (clamped to `u64::MAX`).
    pub fn record_duration(&self, duration: core::time::Duration) {
        let nanos = duration.as_nanos();
        let v_u64 = if nanos > u128::from(u64::MAX) {
            u64::MAX
        } else {
            u64::try_from(nanos).unwrap_or(u64::MAX)
        };
        // Clamp to HDR bounds
        self.record(v_u64);
    }

    #[inline]
    /// Minimum recorded value, if any.
    pub fn min(&self) -> Option<u64> {
        self.inner
            .read()
            .ok()
            .and_then(|h| if h.is_empty() { None } else { Some(h.min()) })
    }

    #[inline]
    /// Maximum recorded value, if any.
    pub fn max(&self) -> Option<u64> {
        self.inner
            .read()
            .ok()
            .and_then(|h| if h.is_empty() { None } else { Some(h.max()) })
    }

    #[inline]
    /// Mean of recorded values, if any.
    pub fn mean(&self) -> Option<f64> {
        self.inner
            .read()
            .ok()
            .and_then(|h| if h.is_empty() { None } else { Some(h.mean()) })
    }

    #[inline]
    /// Number of samples recorded.
    pub fn count(&self) -> u64 {
        self.inner.read().map(|h| h.len()).unwrap_or(0)
    }

    #[inline]
    /// Returns true if no samples have been recorded.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    #[inline]
    /// Value at the given percentile in [0.0, 1.0].
    pub fn percentile(&self, percentile: f64) -> Option<u64> {
        let p = percentile.clamp(0.0, 1.0) * 100.0;
        self.inner.read().ok().and_then(|h| {
            if h.is_empty() {
                None
            } else {
                Some(h.value_at_percentile(p))
            }
        })
    }

    #[inline]
    /// Median (p50).
    pub fn median(&self) -> Option<u64> {
        self.percentile(0.5)
    }

    #[inline]
    /// Median as `Duration`.
    pub fn median_duration(&self) -> Option<core::time::Duration> {
        self.median().map(core::time::Duration::from_nanos)
    }

    #[inline]
    /// Percentile as `Duration`.
    pub fn percentile_duration(&self, p: f64) -> Option<core::time::Duration> {
        self.percentile(p).map(core::time::Duration::from_nanos)
    }

    #[inline]
    /// Batch percentile queries.
    pub fn percentiles(&self, ps: &[f64]) -> Vec<Option<u64>> {
        let Ok(guard) = self.inner.read() else {
            return vec![None; ps.len()];
        };
        if guard.is_empty() {
            return vec![None; ps.len()];
        }
        ps.iter()
            .map(|&p| {
                let pct = p.clamp(0.0, 1.0) * 100.0;
                Some(guard.value_at_percentile(pct))
            })
            .collect()
    }

    #[inline]
    /// Reset the histogram to empty state.
    pub fn reset(&self) {
        if let Ok(mut h) = self.inner.write() {
            h.reset();
        }
    }
}

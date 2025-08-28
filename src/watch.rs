#![cfg(all(feature = "std", feature = "metrics"))]

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::time::Instant;

use crate::histogram::Histogram;

/// Default lowest discernible value (1ns)
const DEFAULT_LOWEST: u64 = 1;
/// Default highest trackable value (~1 hour in ns)
const DEFAULT_HIGHEST: u64 = 3_600_000_000_000;
// Note: precision is fixed internally for performance; no configurable sigfig.

/// Central, thread-safe metrics collector for production timing.
///
/// Holds per-metric internal `Histogram` instances and provides efficient
/// recording and percentile queries via `snapshot()`. Cheap to clone, safe to
/// share across threads and async tasks.
#[derive(Clone)]
pub struct Watch {
    inner: Arc<Inner>,
}

impl Default for Watch {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

struct Inner {
    // Store Arc<Histogram> to allow lock-free record on hot path
    hist: RwLock<HashMap<String, Arc<Histogram>>>,
    lowest: u64,
    highest: u64,
}

/// Snapshot stats for a single metric.
#[derive(Debug, Clone, Copy)]
pub struct WatchStats {
    /// Number of recorded samples.
    pub count: u64,
    /// Minimum observed value (ns).
    pub min: u64,
    /// Maximum observed value (ns).
    pub max: u64,
    /// 50th percentile/median (ns).
    pub p50: u64,
    /// 90th percentile (ns).
    pub p90: u64,
    /// 95th percentile (ns).
    pub p95: u64,
    /// 99th percentile (ns).
    pub p99: u64,
    /// 99.9th percentile (ns).
    pub p999: u64,
    /// Arithmetic mean (ns).
    pub mean: f64,
}

impl fmt::Debug for Watch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Watch")
            .field(
                "metrics_len",
                &self.inner.hist.read().map(|m| m.len()).unwrap_or(0),
            )
            .finish()
    }
}

impl Watch {
    /// Create a new Watch with sensible defaults.
    pub fn new() -> Self {
        Self::with_bounds(DEFAULT_LOWEST, DEFAULT_HIGHEST)
    }

    /// Create a builder to configure histogram bounds and precision.
    #[inline]
    pub fn builder() -> WatchBuilder {
        WatchBuilder::new()
    }

    /// Create a Watch with custom histogram bounds.
    ///
    /// `lowest_discernible`: smallest value discernible (ns)
    /// `highest_trackable`: largest value tracked (ns)
    pub fn with_bounds(lowest_discernible: u64, highest_trackable: u64) -> Self {
        let lowest = lowest_discernible.max(1);
        let highest = highest_trackable.max(lowest + 1);
        Self {
            inner: Arc::new(Inner {
                hist: RwLock::new(HashMap::new()),
                lowest,
                highest,
            }),
        }
    }

    /// Record a duration in nanoseconds for a metric name.
    ///
    /// Safe, thread-safe, and minimal overhead.
    ///
    /// # Panics
    /// Panics if the internal lock is poisoned from a prior panic.
    pub fn record(&self, name: &str, duration_ns: u64) {
        // Clamp to histogram range to avoid errors.
        let ns = duration_ns.clamp(self.inner.lowest, self.inner.highest);

        // Fast path: try obtain Arc without write locking
        if let Ok(map) = self.inner.hist.read() {
            if let Some(h) = map.get(name) {
                h.record(ns);
                return;
            }
        }

        // Slow path: create the histogram under write lock if absent
        let mut map = self.inner.hist.write().expect("watch write lock poisoned");
        let h = map
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Histogram::new()))
            .clone();
        drop(map);
        h.record(ns);
    }

    /// Record elapsed time since `start` for a metric name.
    pub fn record_instant(&self, name: &str, start: Instant) -> u64 {
        let ns = start.elapsed().as_nanos();
        // Convert to u64 safely, saturating at u64::MAX
        let ns_u64 = u64::try_from(ns).unwrap_or(u64::MAX);
        self.record(name, ns_u64);
        ns_u64
    }

    /// Return a snapshot of all metrics with basic statistics.
    ///
    /// Implementation clones histograms under a read lock, then computes outside the lock
    /// to minimize lock hold times and contention.
    ///
    /// # Panics
    /// Panics if the internal lock is poisoned from a prior panic.
    pub fn snapshot(&self) -> HashMap<String, WatchStats> {
        let items: Vec<(String, Arc<Histogram>)> = {
            let map = self.inner.hist.read().expect("watch read lock poisoned");
            map.iter()
                .map(|(k, v)| (k.clone(), Arc::clone(v)))
                .collect()
        };

        let mut out = HashMap::with_capacity(items.len());
        for (name, h) in items {
            let count = h.count();
            if count == 0 {
                out.insert(
                    name,
                    WatchStats {
                        count: 0,
                        min: 0,
                        max: 0,
                        p50: 0,
                        p90: 0,
                        p95: 0,
                        p99: 0,
                        p999: 0,
                        mean: 0.0,
                    },
                );
                continue;
            }

            // Safe unwraps since count > 0
            let min = h.min().unwrap_or(0);
            let max = h.max().unwrap_or(0);
            let p50 = h.percentile(0.50).unwrap_or(min);
            let p90 = h.percentile(0.90).unwrap_or(max);
            let p95 = h.percentile(0.95).unwrap_or(max);
            let p99 = h.percentile(0.99).unwrap_or(max);
            let p999 = h.percentile(0.999).unwrap_or(max);
            let mean = h.mean().unwrap_or(0.0);

            out.insert(
                name,
                WatchStats {
                    count,
                    min,
                    max,
                    p50,
                    p90,
                    p95,
                    p99,
                    p999,
                    mean,
                },
            );
        }
        out
    }

    /// Clear all metrics.
    ///
    /// # Panics
    /// Panics if the internal lock is poisoned from a prior panic.
    pub fn clear(&self) {
        let mut map = self.inner.hist.write().expect("watch write lock poisoned");
        map.clear();
    }

    /// Clear a specific metric by name.
    ///
    /// # Panics
    /// Panics if the internal lock is poisoned from a prior panic.
    pub fn clear_name(&self, name: &str) {
        let mut map = self.inner.hist.write().expect("watch write lock poisoned");
        map.remove(name);
    }
}

/// Builder for configuring and constructing a `Watch`.
#[derive(Debug, Clone, Copy)]
pub struct WatchBuilder {
    lowest: u64,
    highest: u64,
}

impl Default for WatchBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl WatchBuilder {
    /// Start a builder with default bounds: 1ns..~1h, 3 significant figures.
    #[inline]
    pub fn new() -> Self {
        Self {
            lowest: DEFAULT_LOWEST,
            highest: DEFAULT_HIGHEST,
        }
    }

    /// Set the lowest discernible value in nanoseconds (min 1ns).
    #[inline]
    #[must_use]
    pub fn lowest(mut self, ns: u64) -> Self {
        self.lowest = ns.max(1);
        self
    }

    /// Set the highest trackable value in nanoseconds (must be > lowest).
    #[inline]
    #[must_use]
    pub fn highest(mut self, ns: u64) -> Self {
        self.highest = ns;
        self
    }

    /// Build the `Watch` with the configured settings.
    #[inline]
    pub fn build(self) -> Watch {
        let lowest = self.lowest.max(1);
        let highest = self.highest.max(lowest + 1);
        Watch::with_bounds(lowest, highest)
    }
}

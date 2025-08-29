#![cfg(all(feature = "std", feature = "metrics"))]

#[cfg(feature = "parking-lot-locks")]
use parking_lot::RwLock;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
#[cfg(not(feature = "parking-lot-locks"))]
use std::sync::RwLock;
use std::time::Instant;

use crate::histogram::Histogram;

// Normalize guard types across lock backends at module scope
#[cfg(feature = "parking-lot-locks")]
type ReadGuard<'a> = parking_lot::RwLockReadGuard<'a, HashMap<Arc<str>, Arc<Histogram>>>;
#[cfg(not(feature = "parking-lot-locks"))]
type ReadGuard<'a> = std::sync::RwLockReadGuard<'a, HashMap<Arc<str>, Arc<Histogram>>>;

#[cfg(feature = "parking-lot-locks")]
type WriteGuard<'a> = parking_lot::RwLockWriteGuard<'a, HashMap<Arc<str>, Arc<Histogram>>>;
#[cfg(not(feature = "parking-lot-locks"))]
type WriteGuard<'a> = std::sync::RwLockWriteGuard<'a, HashMap<Arc<str>, Arc<Histogram>>>;

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
///
/// # Examples
/// Basic record and snapshot:
/// ```
/// use benchmark::Watch;
/// let w = Watch::new();
/// w.record("op", 1_500);
/// let s = &w.snapshot()["op"];
/// assert_eq!(s.count, 1);
/// assert_eq!(s.min, 1_500);
/// ```
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
    // Keyed by Arc<str> to avoid repeated String allocations and enable cheap sharing.
    hist: RwLock<HashMap<Arc<str>, Arc<Histogram>>>,
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
        let len = self.read_hist().len();
        f.debug_struct("Watch").field("metrics_len", &len).finish()
    }
}

impl Watch {
    #[cfg(feature = "parking-lot-locks")]
    #[inline]
    fn read_hist(&self) -> ReadGuard<'_> {
        self.inner.hist.read()
    }

    #[cfg(not(feature = "parking-lot-locks"))]
    #[inline]
    fn read_hist(&self) -> ReadGuard<'_> {
        self.inner.hist.read().expect("watch read lock poisoned")
    }

    #[cfg(feature = "parking-lot-locks")]
    #[inline]
    fn write_hist(&self) -> WriteGuard<'_> {
        self.inner.hist.write()
    }

    #[cfg(not(feature = "parking-lot-locks"))]
    #[inline]
    fn write_hist(&self) -> WriteGuard<'_> {
        self.inner.hist.write().expect("watch write lock poisoned")
    }

    /// Create a new Watch with sensible defaults.
    ///
    /// # Examples
    /// ```
    /// use benchmark::Watch;
    /// let w = Watch::new();
    /// // empty initially
    /// assert!(w.snapshot().is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_bounds(DEFAULT_LOWEST, DEFAULT_HIGHEST)
    }

    /// Create a builder to configure histogram bounds and precision.
    ///
    /// # Examples
    /// ```
    /// use benchmark::Watch;
    /// let w = Watch::builder().lowest(10).highest(1_000_000).build();
    /// let _ = w; // built successfully
    /// ```
    #[inline]
    pub fn builder() -> WatchBuilder {
        WatchBuilder::new()
    }

    /// Create a Watch with custom histogram bounds.
    ///
    /// `lowest_discernible`: smallest value discernible (ns)
    /// `highest_trackable`: largest value tracked (ns)
    ///
    /// # Examples
    /// ```
    /// use benchmark::Watch;
    /// let w = Watch::with_bounds(5, 10_000);
    /// let _ = w.snapshot();
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// use benchmark::Watch;
    /// let w = Watch::new();
    /// w.record("t", 42);
    /// assert_eq!(w.snapshot()["t"].count, 1);
    /// ```
    pub fn record(&self, name: &str, duration_ns: u64) {
        // Clamp to histogram range to avoid errors.
        let ns = duration_ns.clamp(self.inner.lowest, self.inner.highest);

        // Fast path: try obtain Arc without write locking
        let existing: Option<Arc<Histogram>> = {
            let map = self.read_hist();
            map.get(name).cloned()
        };
        if let Some(h) = existing {
            h.record(ns);
            return;
        }

        // Slow path: create the histogram under write lock if absent
        let mut map = self.write_hist();
        let key: Arc<str> = Arc::<str>::from(name);
        let h = map
            .entry(key)
            .or_insert_with(|| Arc::new(Histogram::new()))
            .clone();
        h.record(ns);
    }

    /// Record elapsed time since `start` for a metric name.
    ///
    /// # Examples
    /// ```
    /// use benchmark::Watch;
    /// use std::time::Instant;
    /// let w = Watch::new();
    /// let start = Instant::now();
    /// // do work
    /// let ns = w.record_instant("io", start);
    /// assert!(ns >= 0);
    /// ```
    pub fn record_instant(&self, name: &str, start: Instant) -> u64 {
        let ns_u128 = start.elapsed().as_nanos();
        // Convert to u64 using infallible saturating via try_from to satisfy clippy
        let ns_u64 = if ns_u128 > u128::from(u64::MAX) {
            u64::MAX
        } else {
            u64::try_from(ns_u128).unwrap_or(u64::MAX)
        };
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
    ///
    /// # Examples
    /// ```
    /// use benchmark::Watch;
    /// let w = Watch::new();
    /// w.record("rpc", 10);
    /// w.record("rpc", 20);
    /// let m = &w.snapshot()["rpc"];
    /// assert_eq!(m.count, 2);
    /// assert!(m.min <= m.p50 && m.p50 <= m.max);
    /// ```
    pub fn snapshot(&self) -> HashMap<String, WatchStats> {
        let items: Vec<(Arc<str>, Arc<Histogram>)> = {
            let map = self.read_hist();
            map.iter()
                .map(|(k, v)| (Arc::clone(k), Arc::clone(v)))
                .collect()
        };

        let mut out = HashMap::with_capacity(items.len());
        for (name, h) in items {
            let count = h.count();
            if count == 0 {
                out.insert(
                    name.to_string(),
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
                name.to_string(),
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
    ///
    /// # Examples
    /// ```
    /// use benchmark::Watch;
    /// let w = Watch::new();
    /// w.record("a", 1);
    /// assert!(!w.snapshot().is_empty());
    /// w.clear();
    /// assert!(w.snapshot().is_empty());
    /// ```
    pub fn clear(&self) {
        let mut map = self.write_hist();
        map.clear();
    }

    /// Clear a specific metric by name.
    ///
    /// # Panics
    /// Panics if the internal lock is poisoned from a prior panic.
    ///
    /// # Examples
    /// ```
    /// use benchmark::Watch;
    /// let w = Watch::new();
    /// w.record("x", 1);
    /// w.clear_name("x");
    /// assert!(!w.snapshot().contains_key("x"));
    /// ```
    pub fn clear_name(&self, name: &str) {
        let mut map = self.write_hist();
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
    ///
    /// # Examples
    /// ```
    /// use benchmark::WatchBuilder;
    /// let _w = WatchBuilder::new().lowest(1).highest(1_000_000).build();
    /// ```
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

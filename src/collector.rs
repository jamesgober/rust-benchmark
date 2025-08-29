//! Thread-safe collection of measurements.

use crate::{Duration, Measurement};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Basic statistics for a set of measurements.
///
/// # Examples
/// ```
/// use benchmark::{Collector, Duration, Measurement};
///
/// // Collect three measurements for the same name
/// let c = Collector::new();
/// c.record_duration("op", Duration::from_nanos(1_000));
/// c.record_duration("op", Duration::from_nanos(2_000));
/// c.record_duration("op", Duration::from_nanos(3_000));
///
/// let s = c.stats("op").unwrap();
/// assert_eq!(s.count, 3);
/// assert_eq!(s.min.as_nanos(), 1_000);
/// assert_eq!(s.max.as_nanos(), 3_000);
/// assert_eq!(s.mean.as_nanos(), 2_000);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stats {
    /// Number of measurements.
    pub count: u64,
    /// Total duration of all measurements.
    pub total: Duration,
    /// Minimum duration.
    pub min: Duration,
    /// Maximum duration.
    pub max: Duration,
    /// Mean (average) duration.
    pub mean: Duration,
}

/// A thread-safe collector for measurements.
///
/// This collector uses an `Arc<RwLock<HashMap>>` to allow multiple threads
/// to record measurements concurrently. The collector can be cloned to share
/// across threads.
#[derive(Clone, Debug)]
pub struct Collector {
    measurements: Arc<RwLock<HashMap<&'static str, Vec<Duration>>>>,
}

impl Collector {
    /// Creates a new collector.
    ///
    /// # Examples
    /// ```
    /// use benchmark::Collector;
    /// let c = Collector::new();
    /// // initially empty
    /// assert!(c.stats("missing").is_none());
    /// ```
    pub fn new() -> Self {
        Self {
            measurements: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Creates a new collector with pre-allocated capacity.
    ///
    /// This can reduce rehashing when you know the number of metric names.
    ///
    /// # Examples
    /// ```
    /// use benchmark::Collector;
    /// let _c = Collector::with_capacity(32);
    /// ```
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            measurements: Arc::new(RwLock::new(HashMap::with_capacity(capacity))),
        }
    }

    /// Records a measurement.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
    ///
    /// # Examples
    /// ```
    /// use benchmark::{Collector, Duration, Measurement};
    /// let c = Collector::new();
    /// let m = Measurement::new("work", Duration::from_nanos(123), 0);
    /// c.record(&m);
    /// assert_eq!(c.stats("work").unwrap().count, 1);
    /// ```
    pub fn record(&self, measurement: &Measurement) {
        let mut lock = self.measurements.write().unwrap();
        lock.entry(measurement.name)
            .or_default()
            .push(measurement.duration);
    }

    /// Records a duration directly.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
    ///
    /// # Examples
    /// ```
    /// use benchmark::{Collector, Duration};
    /// let c = Collector::new();
    /// c.record_duration("db_query", Duration::from_nanos(5_000));
    /// let s = c.stats("db_query").unwrap();
    /// assert_eq!(s.count, 1);
    /// assert_eq!(s.total.as_nanos(), 5_000);
    /// ```
    pub fn record_duration(&self, name: &'static str, duration: Duration) {
        let mut lock = self.measurements.write().unwrap();
        lock.entry(name).or_default().push(duration);
    }

    /// Gets statistics for a named measurement.
    ///
    /// Returns `None` if no measurements exist for the given name.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
    ///
    /// # Examples
    /// ```
    /// use benchmark::{Collector, Duration};
    /// let c = Collector::new();
    /// assert!(c.stats("x").is_none());
    /// c.record_duration("x", Duration::from_nanos(10));
    /// c.record_duration("x", Duration::from_nanos(20));
    /// let s = c.stats("x").unwrap();
    /// assert_eq!(s.count, 2);
    /// ```
    pub fn stats(&self, name: &str) -> Option<Stats> {
        // Clone the vector under a read lock to minimize lock hold time, then compute outside the lock
        let durations: Vec<Duration> = {
            let lock = self.measurements.read().unwrap();
            lock.get(name)?.clone()
        };

        if durations.is_empty() {
            return None;
        }

        // Single pass: compute total, min, max
        let mut iter = durations.iter().copied();
        let first = iter.next()?;
        let mut total: u128 = first.as_nanos();
        let mut min = first;
        let mut max = first;
        for d in iter {
            let n = d.as_nanos();
            total = total.saturating_add(n);
            if d < min {
                min = d;
            }
            if d > max {
                max = d;
            }
        }

        let count = durations.len() as u64;
        let mean = Duration::from_nanos(total / u128::from(count));
        Some(Stats {
            count,
            total: Duration::from_nanos(total),
            min,
            max,
            mean,
        })
    }

    /// Gets statistics for all measurements.
    ///
    /// Returns a vector of (name, stats) pairs.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
    ///
    /// # Examples
    /// ```
    /// use benchmark::{Collector, Duration};
    /// let c = Collector::new();
    /// c.record_duration("a", Duration::from_nanos(1));
    /// c.record_duration("b", Duration::from_nanos(2));
    /// let mut all = c.all_stats();
    /// all.sort_by(|l, r| l.0.cmp(&r.0));
    /// assert_eq!(all.len(), 2);
    /// assert_eq!(all[0].0, "a");
    /// ```
    pub fn all_stats(&self) -> Vec<(String, Stats)> {
        // Snapshot names and their vectors under a read lock, then compute outside to avoid nested locking
        let snapshot: Vec<(&'static str, Vec<Duration>)> = {
            let lock = self.measurements.read().unwrap();
            lock.iter().map(|(&name, v)| (name, v.clone())).collect()
        };

        let mut out = Vec::with_capacity(snapshot.len());
        for (name, durations) in snapshot {
            if durations.is_empty() {
                continue;
            }

            // Single pass per key
            let mut iter = durations.iter().copied();
            if let Some(first) = iter.next() {
                let mut total: u128 = first.as_nanos();
                let mut min = first;
                let mut max = first;
                for d in iter {
                    let n = d.as_nanos();
                    total = total.saturating_add(n);
                    if d < min {
                        min = d;
                    }
                    if d > max {
                        max = d;
                    }
                }
                let count = durations.len() as u64;
                let mean = Duration::from_nanos(total / u128::from(count));
                out.push((
                    name.to_string(),
                    Stats {
                        count,
                        total: Duration::from_nanos(total),
                        min,
                        max,
                        mean,
                    },
                ));
            }
        }
        out
    }

    /// Clears all measurements.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
    ///
    /// # Examples
    /// ```
    /// use benchmark::{Collector, Duration};
    /// let c = Collector::new();
    /// c.record_duration("t", Duration::from_nanos(1));
    /// assert!(c.stats("t").is_some());
    /// c.clear();
    /// assert!(c.stats("t").is_none());
    /// ```
    pub fn clear(&self) {
        let mut lock = self.measurements.write().unwrap();
        lock.clear();
    }

    /// Clears measurements for a specific name.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
    ///
    /// # Examples
    /// ```
    /// use benchmark::{Collector, Duration};
    /// let c = Collector::new();
    /// c.record_duration("x", Duration::from_nanos(1));
    /// c.clear_name("x");
    /// assert!(c.stats("x").is_none());
    /// ```
    pub fn clear_name(&self, name: &str) {
        let mut lock = self.measurements.write().unwrap();
        lock.remove(name);
    }
}

impl Default for Collector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_basic() {
        let collector = Collector::new();

        collector.record_duration("test", Duration::from_nanos(1000));
        collector.record_duration("test", Duration::from_nanos(2000));
        collector.record_duration("test", Duration::from_nanos(3000));

        let stats = collector.stats("test").unwrap();
        assert_eq!(stats.count, 3);
        assert_eq!(stats.total.as_nanos(), 6000);
        assert_eq!(stats.min.as_nanos(), 1000);
        assert_eq!(stats.max.as_nanos(), 3000);
        assert_eq!(stats.mean.as_nanos(), 2000);
    }

    #[test]
    fn test_collector_multiple_names() {
        let collector = Collector::new();

        collector.record_duration("foo", Duration::from_nanos(100));
        collector.record_duration("bar", Duration::from_nanos(200));

        assert!(collector.stats("foo").is_some());
        assert!(collector.stats("bar").is_some());
        assert!(collector.stats("baz").is_none());

        let all = collector.all_stats();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_collector_thread_safety() {
        use std::thread;

        let collector = Arc::new(Collector::new());
        let mut handles = vec![];

        for i in 0u64..10 {
            let c = Arc::clone(&collector);
            let handle = thread::spawn(move || {
                for j in 0u64..10 {
                    c.record_duration("test", Duration::from_nanos(u128::from(i * 10 + j)));
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let stats = collector.stats("test").unwrap();
        assert_eq!(stats.count, 100);
    }

    #[test]
    fn test_collector_clear() {
        let collector = Collector::new();

        collector.record_duration("test", Duration::from_nanos(1000));
        assert!(collector.stats("test").is_some());

        collector.clear();
        assert!(collector.stats("test").is_none());
    }
}

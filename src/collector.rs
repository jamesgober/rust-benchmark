//! Thread-safe collection of measurements.

use crate::{Duration, Measurement};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Basic statistics for a set of measurements.
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
    pub fn new() -> Self {
        Self { measurements: Arc::new(RwLock::new(HashMap::new())) }
    }

    /// Creates a new collector with pre-allocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self { measurements: Arc::new(RwLock::new(HashMap::with_capacity(capacity))) }
    }

    /// Records a measurement.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
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
    pub fn stats(&self, name: &str) -> Option<Stats> {
        let lock = self.measurements.read().unwrap();
        let durations = lock.get(name)?;

        if durations.is_empty() {
            return None;
        }

        let count = durations.len() as u64;
        let total: u128 = durations.iter().map(Duration::as_nanos).sum();
        let min = durations.iter().min().copied().unwrap_or(Duration::ZERO);
        let max = durations.iter().max().copied().unwrap_or(Duration::ZERO);
        let mean = Duration::from_nanos(total / u128::from(count));

        Some(Stats { count, total: Duration::from_nanos(total), min, max, mean })
    }

    /// Gets statistics for all measurements.
    ///
    /// Returns a vector of (name, stats) pairs.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
    pub fn all_stats(&self) -> Vec<(String, Stats)> {
        let lock = self.measurements.read().unwrap();
        lock.keys()
            .filter_map(|&name| self.stats(name).map(|stats| (name.to_string(), stats)))
            .collect()
    }

    /// Clears all measurements.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
    pub fn clear(&self) {
        let mut lock = self.measurements.write().unwrap();
        lock.clear();
    }

    /// Clears measurements for a specific name.
    ///
    /// # Panics
    ///
    /// Panics if the lock is poisoned.
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

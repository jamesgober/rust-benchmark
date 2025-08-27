//! A zero-dependency, high-performance time measurement library for Rust.
//!
//! This library provides nanosecond precision benchmarking with true zero-overhead
//! when disabled. Designed as a foundational primitive that other libraries can
//! depend on without concern for bloat, version conflicts, or performance impact.

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

#[cfg(feature = "std")]
use std::time::Instant;

/// A duration represented in nanoseconds.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    nanos: u128,
}

impl Duration {
    /// Zero duration constant.
    pub const ZERO: Self = Self { nanos: 0 };

    /// Creates a new Duration from nanoseconds.
    #[inline]
    pub const fn from_nanos(nanos: u128) -> Self {
        Self { nanos }
    }

    /// Returns the number of nanoseconds.
    #[inline]
    pub const fn as_nanos(&self) -> u128 {
        self.nanos
    }

    /// Returns the number of microseconds.
    #[inline]
    pub const fn as_micros(&self) -> u128 {
        self.nanos / 1_000
    }

    /// Returns the number of milliseconds.
    #[inline]
    pub const fn as_millis(&self) -> u128 {
        self.nanos / 1_000_000
    }

    /// Returns the number of seconds as a floating point number.
    #[inline]
    pub fn as_secs_f64(&self) -> f64 {
        self.nanos as f64 / 1_000_000_000.0
    }
}

/// A single time measurement.
#[derive(Clone, Debug)]
pub struct Measurement {
    /// The name of this measurement.
    pub name: &'static str,
    /// The duration of the measurement.
    pub duration: Duration,
    /// Timestamp when measurement was taken (nanoseconds since start).
    pub timestamp: u128,
}

/// Basic statistics for a set of measurements.
#[derive(Clone, Debug)]
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

/// Measures the execution time of a function.
///
/// # Examples
/// ```
/// use benchmark::{measure, Duration};
///
/// let (result, duration) = measure(|| {
///     // Some computation
///     42
/// });
/// assert_eq!(result, 42);
/// assert!(duration.as_nanos() > 0);
/// ```
#[cfg(all(feature = "std", feature = "enabled"))]
#[inline]
pub fn measure<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let duration = Duration::from_nanos(start.elapsed().as_nanos());
    (result, duration)
}

/// Measures the execution time of a function (disabled version).
#[cfg(not(all(feature = "std", feature = "enabled")))]
#[inline(always)]
pub fn measure<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    (f(), Duration::ZERO)
}

/// Measures the execution time of a function with a name.
///
/// # Examples
/// ```
/// use benchmark::{measure_named, Duration};
///
/// let (result, measurement) = measure_named("computation", || {
///     // Some computation
///     42
/// });
/// assert_eq!(result, 42);
/// assert_eq!(measurement.name, "computation");
/// ```
#[cfg(all(feature = "std", feature = "enabled"))]
#[inline]
pub fn measure_named<T, F: FnOnce() -> T>(name: &'static str, f: F) -> (T, Measurement) {
    // Avoid unsupported syscalls under Miri isolation
    let timestamp = if cfg!(miri) {
        0
    } else {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    };

    let start = Instant::now();
    let result = f();
    let duration = Duration::from_nanos(start.elapsed().as_nanos());

    let measurement = Measurement {
        name,
        duration,
        timestamp,
    };

    (result, measurement)
}

#[cfg(not(all(feature = "std", feature = "enabled")))]
#[inline(always)]
pub fn measure_named<T, F: FnOnce() -> T>(name: &'static str, f: F) -> (T, Measurement) {
    let measurement = Measurement {
        name,
        duration: Duration::ZERO,
        timestamp: 0,
    };
    (f(), measurement)
}

/// Times an expression and returns (result, duration).
#[cfg(feature = "enabled")]
#[macro_export]
macro_rules! time {
    ($expr:expr) => {{
        $crate::measure(|| $expr)
    }};
}

/// Times an expression and returns (result, duration) - disabled version.
#[cfg(not(feature = "enabled"))]
#[macro_export]
macro_rules! time {
    ($expr:expr) => {{
        ($expr, $crate::Duration::ZERO)
    }};
}

/// Times an expression with a name and returns (result, measurement).
#[cfg(feature = "enabled")]
#[macro_export]
macro_rules! time_named {
    ($name:expr, $expr:expr) => {{
        $crate::measure_named($name, || $expr)
    }};
}

/// Times an expression with a name and returns (result, measurement) - disabled version.
#[cfg(not(feature = "enabled"))]
#[macro_export]
macro_rules! time_named {
    ($name:expr, $expr:expr) => {{
        let measurement = $crate::Measurement {
            name: $name,
            duration: $crate::Duration::ZERO,
            timestamp: 0,
        };
        ($expr, measurement)
    }};
}

#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
use std::sync::{Arc, RwLock};

/// A thread-safe collector for measurements.
#[cfg(feature = "std")]
#[derive(Clone, Debug)]
pub struct Collector {
    measurements: Arc<RwLock<HashMap<&'static str, Vec<Duration>>>>,
}

#[cfg(feature = "std")]
impl Collector {
    /// Creates a new collector.
    pub fn new() -> Self {
        Self {
            measurements: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Records a measurement.
    pub fn record(&self, measurement: Measurement) {
        let mut lock = self.measurements.write().unwrap();
        lock.entry(measurement.name)
            .or_default()
            .push(measurement.duration);
    }

    /// Gets statistics for a named measurement.
    pub fn stats(&self, name: &str) -> Option<Stats> {
        let lock = self.measurements.read().unwrap();
        let durations = lock.get(name)?;

        if durations.is_empty() {
            return None;
        }

        let count = durations.len() as u64;
        let total: u128 = durations.iter().map(|d| d.as_nanos()).sum();
        let min = durations.iter().min().copied().unwrap_or(Duration::ZERO);
        let max = durations.iter().max().copied().unwrap_or(Duration::ZERO);
        let mean = Duration::from_nanos(total / count as u128);

        Some(Stats {
            count,
            total: Duration::from_nanos(total),
            min,
            max,
            mean,
        })
    }

    /// Gets statistics for all measurements.
    pub fn all_stats(&self) -> Vec<(String, Stats)> {
        let lock = self.measurements.read().unwrap();
        lock.keys()
            .filter_map(|&name| self.stats(name).map(|stats| (name.to_string(), stats)))
            .collect()
    }
}

#[cfg(feature = "std")]
impl Default for Collector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::*;

    #[test]
    fn test_duration() {
        let d = Duration::from_nanos(1_500_000_000);
        assert_eq!(d.as_nanos(), 1_500_000_000);
        assert_eq!(d.as_micros(), 1_500_000);
        assert_eq!(d.as_millis(), 1_500);
        assert!((d.as_secs_f64() - 1.5).abs() < 0.0001);
    }

    #[test]
    fn test_measure() {
        let (result, duration) = measure(|| {
            #[cfg(feature = "enabled")]
            {
                // Ensure non-zero elapsed time on platforms with coarse timers
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
            42
        });
        assert_eq!(result, 42);
        #[cfg(feature = "enabled")]
        assert!(duration.as_nanos() > 0);
        #[cfg(not(feature = "enabled"))]
        assert_eq!(duration.as_nanos(), 0);
    }

    #[test]
    fn test_collector() {
        let collector = Collector::new();
        let measurement = Measurement {
            name: "test",
            duration: Duration::from_nanos(1000),
            timestamp: 0,
        };

        collector.record(measurement.clone());
        collector.record(Measurement {
            name: "test",
            duration: Duration::from_nanos(2000),
            timestamp: 1,
        });

        let stats = collector.stats("test").unwrap();
        assert_eq!(stats.count, 2);
        assert_eq!(stats.min.as_nanos(), 1000);
        assert_eq!(stats.max.as_nanos(), 2000);
        assert_eq!(stats.mean.as_nanos(), 1500);
    }
}

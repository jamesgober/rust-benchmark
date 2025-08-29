//! Measurement type for representing a timed operation.

use crate::Duration;

/// A single time measurement.
///
/// Contains the name of the operation, how long it took, and when it was measured.
#[derive(Clone, Debug)]
pub struct Measurement {
    /// The name of this measurement.
    pub name: &'static str,
    /// The duration of the measurement.
    pub duration: Duration,
    /// Timestamp when measurement was taken (nanoseconds since UNIX epoch).
    pub timestamp: u128,
}

impl Measurement {
    /// Creates a new measurement.
    ///
    /// # Examples
    /// ```
    /// use benchmark::{Measurement, Duration};
    /// let m = Measurement::new("op", Duration::from_nanos(123), 1);
    /// assert_eq!(m.name, "op");
    /// assert_eq!(m.duration.as_nanos(), 123);
    /// assert_eq!(m.timestamp, 1);
    /// ```
    pub fn new(name: &'static str, duration: Duration, timestamp: u128) -> Self {
        Self {
            name,
            duration,
            timestamp,
        }
    }

    /// Creates a new measurement with zero duration and timestamp.
    ///
    /// # Examples
    /// ```
    /// use benchmark::Measurement;
    /// let m = Measurement::zero("noop");
    /// assert_eq!(m.name, "noop");
    /// assert_eq!(m.duration.as_nanos(), 0);
    /// assert_eq!(m.timestamp, 0);
    /// ```
    pub fn zero(name: &'static str) -> Self {
        Self {
            name,
            duration: Duration::ZERO,
            timestamp: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measurement_creation() {
        let m = Measurement::new("test", Duration::from_nanos(1000), 123_456);
        assert_eq!(m.name, "test");
        assert_eq!(m.duration.as_nanos(), 1000);
        assert_eq!(m.timestamp, 123_456);
    }

    #[test]
    fn test_measurement_zero() {
        let m = Measurement::zero("test");
        assert_eq!(m.name, "test");
        assert_eq!(m.duration.as_nanos(), 0);
        assert_eq!(m.timestamp, 0);
    }
}

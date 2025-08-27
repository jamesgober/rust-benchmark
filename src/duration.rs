//! Duration type for representing time measurements.

use core::fmt;

/// A duration represented in nanoseconds.
///
/// This type uses a single `u128` field to store nanoseconds, providing
/// 584 years of range with nanosecond precision. This design is optimized
/// for simplicity and cache efficiency.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Duration {
    pub(crate) nanos: u128,
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
    #[allow(clippy::cast_precision_loss)]
    pub fn as_secs_f64(&self) -> f64 {
        self.nanos as f64 / 1_000_000_000.0
    }

    /// Returns the number of seconds as a floating point number.
    #[inline]
    #[allow(clippy::cast_precision_loss)]
    pub fn as_secs_f32(&self) -> f32 {
        self.nanos as f32 / 1_000_000_000.0
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nanos = self.as_nanos();

        if nanos == 0 {
            write!(f, "0ns")
        } else if nanos < 1_000 {
            write!(f, "{nanos}ns")
        } else if nanos < 1_000_000 {
            #[allow(clippy::cast_precision_loss)]
            {
                write!(f, "{:.2}µs", nanos as f64 / 1_000.0)
            }
        } else if nanos < 1_000_000_000 {
            #[allow(clippy::cast_precision_loss)]
            {
                write!(f, "{:.2}ms", nanos as f64 / 1_000_000.0)
            }
        } else if nanos < 60_000_000_000 {
            #[allow(clippy::cast_precision_loss)]
            {
                write!(f, "{:.2}s", nanos as f64 / 1_000_000_000.0)
            }
        } else {
            let secs = nanos / 1_000_000_000;
            let mins = secs / 60;
            let secs = secs % 60;
            write!(f, "{mins}m {secs}s")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duration_conversions() {
        let d = Duration::from_nanos(1_234_567_890);
        assert_eq!(d.as_nanos(), 1_234_567_890);
        assert_eq!(d.as_micros(), 1_234_567);
        assert_eq!(d.as_millis(), 1_234);
        assert!((d.as_secs_f64() - 1.234_567_89).abs() < 0.000_000_1);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_duration_display() {
        assert_eq!(Duration::from_nanos(0).to_string(), "0ns");
        assert_eq!(Duration::from_nanos(123).to_string(), "123ns");
        assert_eq!(Duration::from_nanos(1_500).to_string(), "1.50µs");
        assert_eq!(Duration::from_nanos(1_500_000).to_string(), "1.50ms");
        assert_eq!(Duration::from_nanos(1_500_000_000).to_string(), "1.50s");
        assert_eq!(Duration::from_nanos(65_000_000_000).to_string(), "1m 5s");
    }

    #[test]
    fn test_duration_ord() {
        let d1 = Duration::from_nanos(100);
        let d2 = Duration::from_nanos(200);
        let d3 = Duration::from_nanos(100);

        assert!(d1 < d2);
        assert!(d2 > d1);
        assert!(d1 == d3);
    }
}

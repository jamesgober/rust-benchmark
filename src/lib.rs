//! A zero-dependency, high-performance time measurement library for Rust.
//!
//! This library provides nanosecond precision benchmarking with true zero-overhead
//! when disabled. Designed as a foundational primitive that other libraries can
//! depend on without concern for bloat, version conflicts, or performance impact.
//!
//! # Features
//!
//! - **Zero Dependencies**: Built using only the Rust standard library
//! - **True Zero Overhead**: When disabled, all code compiles away completely
//! - **Thread Safe**: Core functions are pure, with optional thread-safe collection
//! - **Async Compatible**: Works seamlessly with any async runtime
//! - **Simple API**: Just four functions and two macros
//!
//! # Quick Start
//!
//! ```rust
//! use benchmark::{measure, time};
//!
//! // Measure a function
//! let (result, duration) = measure(|| {
//!     // Some expensive computation
//!     42
//! });
//! println!("Computation took {} nanoseconds", duration.as_nanos());
//!
//! // Use the macro for convenience
//! fn expensive_function() -> i32 { 2 + 2 }
//! let (result, duration) = time!(expensive_function());
//! assert_eq!(result, 4);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

// Core modules
#[cfg(feature = "std")]
mod collector;
mod duration;
mod measurement;

// Public exports
#[cfg(feature = "std")]
pub use collector::{Collector, Stats};
pub use duration::Duration;
pub use measurement::Measurement;

// Re-export macros at crate root
#[doc(hidden)]
pub use crate as benchmark;

// Core timing functionality
#[cfg(feature = "std")]
use std::time::Instant;

/// Measures the execution time of a function.
///
/// Returns a tuple of (result, duration) where result is the return value
/// of the function and duration is how long it took to execute.
///
/// # Examples
/// ```
/// use benchmark::measure;
///
/// let (result, duration) = measure(|| {
///     // Some computation
///     2 + 2
/// });
/// assert_eq!(result, 4);
/// # // Touch duration under enabled to avoid lints and flakiness
/// # #[cfg(feature = "enabled")]
/// # let _ = duration.as_nanos();
/// ```
#[cfg(all(feature = "enabled", feature = "std"))]
#[inline]
pub fn measure<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let duration = Duration::from_nanos(start.elapsed().as_nanos());
    (result, duration)
}

/// Measures the execution time of a function (disabled version).
#[cfg(not(feature = "enabled"))]
#[inline(always)]
pub fn measure<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    (f(), Duration::ZERO)
}

/// Measures the execution time of a function with a name.
///
/// Returns a tuple of (result, measurement) where result is the return value
/// of the function and measurement contains the timing information.
///
/// # Examples
/// ```
/// use benchmark::measure_named;
///
/// let (result, measurement) = measure_named("computation", || {
///     // Some computation
///     2 + 2
/// });
/// assert_eq!(result, 4);
/// assert_eq!(measurement.name, "computation");
/// ```
#[cfg(all(feature = "enabled", feature = "std"))]
#[inline]
pub fn measure_named<T, F: FnOnce() -> T>(name: &'static str, f: F) -> (T, Measurement) {
    #[cfg(miri)]
    let timestamp = 0;
    #[cfg(not(miri))]
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());

    let start = Instant::now();
    let result = f();
    let duration = Duration::from_nanos(start.elapsed().as_nanos());

    let measurement = Measurement { name, duration, timestamp };

    (result, measurement)
}

/// Measures the execution time of a function with a name (disabled version).
#[cfg(not(feature = "enabled"))]
#[inline(always)]
pub fn measure_named<T, F: FnOnce() -> T>(name: &'static str, f: F) -> (T, Measurement) {
    let measurement = Measurement { name, duration: Duration::ZERO, timestamp: 0 };
    (f(), measurement)
}

// Macros

/// Times an expression and returns (result, duration).
///
/// This is a convenience macro that wraps the `measure` function.
///
/// # Examples
/// ```
/// use benchmark::time;
///
/// let (result, duration) = time!(2 + 2);
/// assert_eq!(result, 4);
/// ```
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
///
/// This is a convenience macro that wraps the `measure_named` function.
///
/// # Examples
/// ```
/// use benchmark::time_named;
///
/// let (result, measurement) = time_named!("addition", 2 + 2);
/// assert_eq!(result, 4);
/// assert_eq!(measurement.name, "addition");
/// ```
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
        let measurement =
            $crate::Measurement { name: $name, duration: $crate::Duration::ZERO, timestamp: 0 };
        ($expr, measurement)
    }};
}

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
//!
//! # Production Metrics (feature = "metrics")
//!
//! The following examples compile only when `features = ["std", "metrics"]` are enabled.
//!
//! ```rust
//! # #[cfg(all(feature = "std", feature = "metrics"))]
//! use benchmark::{stopwatch, Watch};
//! #
//! # #[cfg(all(feature = "std", feature = "metrics"))]
//! fn main() {
//!     let watch = Watch::new();
//!     stopwatch!(watch, "work", {
//!         // do work
//!     });
//!     let s = &watch.snapshot()["work"];
//!     assert!(s.count >= 1);
//! }
//! # #[cfg(not(all(feature = "std", feature = "metrics")))]
//! # fn main() {}
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
#[cfg(feature = "collector")]
mod collector;
mod duration;
#[cfg(all(feature = "collector", feature = "metrics"))]
mod hist_backend;
#[cfg(all(feature = "collector", feature = "hdr"))]
mod hist_hdr;
#[cfg(feature = "collector")]
pub mod histogram;
mod measurement;
#[cfg(feature = "trace")]
mod trace;
#[cfg(feature = "metrics")]
mod timer;
#[cfg(feature = "metrics")]
mod watch;

// Public exports
#[cfg(feature = "collector")]
pub use collector::{Collector, Stats};
pub use duration::Duration;
pub use measurement::Measurement;
#[cfg(feature = "metrics")]
pub use timer::Timer;
#[cfg(feature = "metrics")]
pub use watch::{Watch, WatchBuilder, WatchStats};

// Re-export macros at crate root
#[doc(hidden)]
pub use crate as benchmark;

// Core timing functionality
#[cfg(feature = "benchmark")]
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
/// # // Touch duration under benchmark to avoid lints and flakiness
/// # #[cfg(feature = "benchmark")]
/// # let _ = duration.as_nanos();
/// ```
#[cfg(feature = "benchmark")]
#[inline]
pub fn measure<T, F: FnOnce() -> T>(f: F) -> (T, Duration) {
    let start = Instant::now();
    let result = f();
    let duration = Duration::from_nanos(start.elapsed().as_nanos());
    (result, duration)
}

/// Measures the execution time of a function (disabled version).
#[cfg(not(feature = "benchmark"))]
#[inline]
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
#[cfg(all(feature = "benchmark", feature = "std"))]
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

    let measurement = Measurement {
        name,
        duration,
        timestamp,
    };

    (result, measurement)
}

/// Measures the execution time of a function with a name (disabled version).
#[cfg(not(feature = "benchmark"))]
#[inline]
pub fn measure_named<T, F: FnOnce() -> T>(name: &'static str, f: F) -> (T, Measurement) {
    let measurement = Measurement {
        name,
        duration: Duration::ZERO,
        timestamp: 0,
    };
    (f(), measurement)
}

// Macros

/// Times an expression and returns (result, duration).
///
/// When features `benchmark` + `std` are active, the macro inlines timing using
/// `std::time::Instant` so it can be used inside async contexts (supports `await`).
///
/// # Examples
/// ```
/// use benchmark::time;
///
/// let (result, duration) = time!(2 + 2);
/// assert_eq!(result, 4);
/// ```
#[cfg(feature = "benchmark")]
#[macro_export]
macro_rules! time {
    ($expr:expr $(,)?) => {{
        let __start = ::std::time::Instant::now();
        let __out = { $expr };
        let __dur = $crate::Duration::from_nanos(__start.elapsed().as_nanos());
        (__out, __dur)
    }};
}

/// Times an expression and returns (result, duration) - disabled version.
#[cfg(not(feature = "benchmark"))]
#[macro_export]
macro_rules! time {
    ($expr:expr $(,)?) => {{
        ($expr, $crate::Duration::ZERO)
    }};
}

/// Times an expression with a name and returns (result, measurement).
///
/// When features `benchmark` + `std` are active, the macro inlines timing using
/// `std::time::Instant` so it can be used inside async contexts (supports `await`).
///
/// # Examples
/// ```
/// use benchmark::time_named;
///
/// let (result, measurement) = time_named!("addition", 2 + 2);
/// assert_eq!(result, 4);
/// assert_eq!(measurement.name, "addition");
/// ```
#[cfg(feature = "benchmark")]
#[macro_export]
macro_rules! time_named {
    ($name:expr, $expr:expr $(,)?) => {{
        let __name: &'static str = $name;
        let __start = ::std::time::Instant::now();
        let __out = { $expr };
        let __dur = $crate::Duration::from_nanos(__start.elapsed().as_nanos());
        #[cfg(miri)]
        let __ts = 0;
        #[cfg(not(miri))]
        let __ts = ::std::time::SystemTime::now()
            .duration_since(::std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_nanos());
        let __measurement = $crate::Measurement {
            name: __name,
            duration: __dur,
            timestamp: __ts,
        };
        (__out, __measurement)
    }};
}

/// Times an expression with a name and returns (result, measurement) - disabled version.
#[cfg(not(feature = "benchmark"))]
#[macro_export]
macro_rules! time_named {
    ($name:expr, $expr:expr $(,)?) => {{
        let measurement = $crate::Measurement {
            name: $name,
            duration: $crate::Duration::ZERO,
            timestamp: 0,
        };
        ($expr, measurement)
    }};
}

/// Stopwatch macro for production metrics collection.
///
/// When features `metrics` + `std` are active, this macro creates a `Timer`
/// which starts immediately before evaluating the body, and records the
/// duration when dropped at the end of the scope. Body may contain `await`.
///
/// Disabled path evaluates body with zero overhead.
#[cfg(feature = "metrics")]
#[macro_export]
macro_rules! stopwatch {
    ($watch:expr, $name:expr, { $($body:tt)* } $(,)?) => {{
        let __timer = $crate::Timer::new($watch.clone(), $name);
        { $($body)* }
    }};
}

/// Disabled version of `stopwatch!` when `metrics` is off.
#[cfg(not(all(feature = "metrics", feature = "std")))]
#[macro_export]
macro_rules! stopwatch {
    ($watch:expr, $name:expr, { $($body:tt)* } $(,)?) => {{
        { $($body)* }
    }};
}

/// Micro-benchmark a code block for a number of iterations and return raw per-iteration durations.
///
/// Two forms are supported:
/// - `benchmark_block!({ body })` uses a default of 10,000 iterations
/// - `benchmark_block!(iters, { body })` runs the block `iters` times
///
/// The block may contain `await` and arbitrary statements. When the `benchmark`
/// feature is disabled, the block executes once (to preserve side effects) and
/// the macro returns an empty `Vec` with zero timing overhead.
#[cfg(feature = "benchmark")]
#[macro_export]
macro_rules! benchmark_block {
    ({ $($body:tt)* } $(,)?) => {
        $crate::benchmark_block!(10_000usize, { $($body)* })
    };
    ($iters:expr, { $($body:tt)* } $(,)?) => {{
        let __iters: usize = $iters;
        let mut __samples: ::std::vec::Vec<$crate::Duration> = ::std::vec::Vec::with_capacity(__iters);
        let mut __i = 0usize;
        while __i < __iters {
            let __start = ::std::time::Instant::now();
            { $($body)* }
            let __dur = $crate::Duration::from_nanos(__start.elapsed().as_nanos());
            __samples.push(__dur);
            __i += 1;
        }
        __samples
    }};
}

/// Disabled version of `benchmark_block!` when `benchmark` is off.
#[cfg(not(feature = "benchmark"))]
#[macro_export]
macro_rules! benchmark_block {
    ({ $($body:tt)* } $(,)?) => {{
        { $($body)* }
        ::std::vec::Vec::<$crate::Duration>::new()
    }};
    ($iters:expr, { $($body:tt)* } $(,)?) => {{
        let _ = $iters; // keep param unused warnings away
        { $($body)* }
        ::std::vec::Vec::<$crate::Duration>::new()
    }};
}

/// Macro-benchmark an expression/function for a number of iterations and return
/// the last result together with raw per-iteration `Measurement`s.
///
/// Forms supported:
/// - `benchmark!(name, expr)` uses a default of 10,000 iterations
/// - `benchmark!(name, iters, expr)` runs `expr` `iters` times
/// - `benchmark!(name, { body })` and `benchmark!(name, iters, { body })` also work
///
/// The expression/body may contain `await`. When the `benchmark` feature is
/// disabled, the expression executes once and the macro returns `(Some(output), vec![])`
/// with zero timing overhead.
#[cfg(feature = "benchmark")]
#[macro_export]
macro_rules! benchmark {
    ($name:expr, { $($body:tt)* } $(,)?) => {
        $crate::benchmark!($name, 10_000usize, { $($body)* })
    };
    ($name:expr, $iters:expr, { $($body:tt)* } $(,)?) => {{
        let __name: &'static str = $name;
        let __iters: usize = $iters;
        let mut __measurements: ::std::vec::Vec<$crate::Measurement> = ::std::vec::Vec::with_capacity(__iters);
        let mut __last = None;
        let mut __i = 0usize;
        while __i < __iters {
            let __start = ::std::time::Instant::now();
            let __out = { $($body)* };
            let __dur = $crate::Duration::from_nanos(__start.elapsed().as_nanos());
            #[cfg(miri)]
            let __ts = 0;
            #[cfg(not(miri))]
            let __ts = ::std::time::SystemTime::now()
                .duration_since(::std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos());
            __measurements.push($crate::Measurement { name: __name, duration: __dur, timestamp: __ts });
            __last = Some(__out);
            __i += 1;
        }
        (__last, __measurements)
    }};
    ($name:expr, $expr:expr $(,)?) => {
        $crate::benchmark!($name, 10_000usize, { $expr })
    };
    ($name:expr, $iters:expr, $expr:expr $(,)?) => {
        $crate::benchmark!($name, $iters, { $expr })
    };
}

/// Disabled version of `benchmark!` when `benchmark` is off.
#[cfg(not(feature = "benchmark"))]
#[macro_export]
macro_rules! benchmark {
    ($name:expr, { $($body:tt)* } $(,)?) => {{
        let _ = $name;
        let __out = { $($body:tt)* };
        (Some(__out), ::std::vec::Vec::<$crate::Measurement>::new())
    }};
    ($name:expr, $iters:expr, { $($body:tt)* } $(,)?) => {{
        let _ = ($name, $iters);
        let __out = { $($body:tt)* };
        (Some(__out), ::std::vec::Vec::<$crate::Measurement>::new())
    }};
    ($name:expr, $expr:expr $(,)?) => {{
        let _ = $name;
        let __out = $expr;
        (Some(__out), ::std::vec::Vec::<$crate::Measurement>::new())
    }};
    ($name:expr, $iters:expr, $expr:expr $(,)?) => {{
        let _ = ($name, $iters);
        let __out = $expr;
        (Some(__out), ::std::vec::Vec::<$crate::Measurement>::new())
    }};
}

// Intentionally no public trace! macro to avoid API surface area.
// Use internal crate::trace::record_event() behind the `trace` feature.

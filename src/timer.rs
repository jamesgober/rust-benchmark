#![cfg(all(feature = "std", feature = "metrics"))]

use std::fmt;
use std::sync::Arc;
use std::time::Instant;

use crate::watch::Watch;

/// A lightweight scope timer that records duration to a central `Watch` on drop.
/// Automatic stop is guaranteed even during unwinding (panic).
///
/// # Examples
/// Basic usage with drop-based recording:
/// ```
/// use benchmark::{Watch, Timer};
/// let w = Watch::new();
/// {
///     let _t = Timer::new(w.clone(), "work");
///     // do work in this scope
/// }
/// // timer dropped, recorded once
/// assert_eq!(w.snapshot()["work"].count, 1);
/// ```
#[must_use]
pub struct Timer {
    watch: Watch,
    name: Arc<str>,
    start: Option<Instant>, // guard to prevent double-record
}

impl fmt::Debug for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timer")
            .field("name", &self.name)
            .field("active", &self.start.is_some())
            .finish_non_exhaustive()
    }
}

impl Timer {
    /// Start a new timer for the given metric name, recording into `watch`.
    ///
    /// # Examples
    /// ```
    /// use benchmark::{Watch, Timer};
    /// let w = Watch::new();
    /// let _t = Timer::new(w, "op");
    /// ```
    #[inline]
    pub fn new(watch: Watch, name: impl Into<Arc<str>>) -> Self {
        Self {
            watch,
            name: name.into(),
            start: Some(Instant::now()),
        }
    }

    /// Stop the timer early and record the duration once.
    /// Returns the recorded nanoseconds.
    ///
    /// Safe to call at most once; subsequent calls are no-ops returning 0.
    ///
    /// # Examples
    /// ```
    /// use benchmark::{Watch, Timer};
    /// let w = Watch::new();
    /// let t = Timer::new(w.clone(), "io");
    /// let ns = t.stop();
    /// assert!(ns >= 0);
    /// // already recorded, drop won't double-record
    /// assert_eq!(w.snapshot()["io"].count, 1);
    /// ```
    #[inline]
    pub fn stop(mut self) -> u64 {
        if let Some(start) = self.start.take() {
            return self.watch.record_instant(&self.name, start);
        }
        0
    }
}

impl Drop for Timer {
    #[inline]
    fn drop(&mut self) {
        if let Some(start) = self.start.take() {
            let _ = self.watch.record_instant(&self.name, start);
        }
    }
}

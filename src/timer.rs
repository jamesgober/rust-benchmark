#![cfg(all(feature = "std", feature = "metrics"))]

use std::fmt;
use std::sync::Arc;
use std::time::Instant;

use crate::watch::Watch;

/// A lightweight scope timer that records duration to a central Watch on drop.
/// Automatic stop is guaranteed even during unwinding (panic).
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

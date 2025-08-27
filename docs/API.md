<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>Benchmark</b>
    <br>
    <sub>
        <sup>API REFERENCE</sup>
    </sub>
    <br>
</h1>

<br>

## Table of Contents
- [Installation](#installation)
- [Features](#features)
- [Types](#types)
  - [Duration](#duration)
  - [Measurement](#measurement)
  - [Stats](#stats)
- [Collector](#collector)
- [Functions](#functions)
  - [measure](#measure)
  - [measure_named](#measure_named)
- [Macros](#macros)
  - [time!](#time)
  - [time_named!](#time_named)
- [Async Usage](#async-usage)
- [Disabled Mode Behavior](#disabled-mode-behavior)

<br><br>

## Installation

### Install Manually

Add this to your `Cargo.toml`:
```toml
[dependencies]
benchmark = "0.2.0"
```

#### Disable Default Features
```toml
[dependencies]
# Disable default features for true zero-overhead
benchmark = { version = "0.2.0", default-features = false }
```

<br>

### Install via Terminal
```bash
# Basic installation (benchmarking feature only)
cargo add benchmark
```

#### Terminal: Disable Default Features
```bash
# Explicitly disabled - zero overhead
cargo add benchmark --no-default-features
```

<hr>
<br>





## Features

- `enabled` (default): enables measurement (otherwise compiles to zero-overhead no-ops)
- `std` (default): uses Rust standard library; disables `no_std`
- `minimal`: minimal build with core timing only (no default features)
- `full`: convenience feature equal to `std + enabled`

<br>

## Types

### Duration
Represents a duration in nanoseconds, backed by a `u128` for wide range and precision.

```rust
use benchmark::Duration;

let d = Duration::from_nanos(1_500);
assert_eq!(d.as_micros(), 1);
assert_eq!(d.to_string(), "1.50µs");
```

- Constructors: `from_nanos(u128)`
- Accessors: `as_nanos() -> u128`, `as_micros() -> u128`, `as_millis() -> u128`, `as_secs_f64() -> f64`, `as_secs_f32() -> f32`
- Constants: `Duration::ZERO`
- Display: human-friendly units (ns/µs/ms/s/m)

<br>

### Measurement
Represents a single named timing with timestamp (nanoseconds since UNIX epoch by default under `std`).

```rust
use benchmark::{Duration, Measurement};

let m = Measurement::new("op", Duration::from_nanos(42), 0);
assert_eq!(m.name, "op");
```

- Fields: `name: &'static str`, `duration: Duration`, `timestamp: u128`
- Constructors: `new(name, duration, timestamp)`, `zero(name)`
- Notes: Timestamps may be `0` in Miri or restricted environments.

<br>

### Stats
Basic statistics for a set of measurements. Available with `std` feature.

- Fields: `count: u64`, `total: Duration`, `min: Duration`, `max: Duration`, `mean: Duration`
- Construction: Returned by `Collector::stats()`/`Collector::all_stats()`.

<br>

## Collector
Thread-safe aggregation of measurements. Available with `std` feature.

```rust
use benchmark::{Collector, Duration, Measurement};

let c = Collector::new();
let m = Measurement::new("op", Duration::from_nanos(10), 0);
c.record(&m); // v0.2.0: takes &Measurement

let stats = c.stats("op").unwrap();
assert_eq!(stats.count, 1);
```

- Constructors: `new()`, `with_capacity(usize)`
- Recording: `record(&Measurement)`, `record_duration(name, Duration)`
- Stats: `stats(name) -> Option<Stats>`, `all_stats() -> Vec<(String, Stats)>`
- Maintenance: `clear()`, `clear_name(name)`
- Concurrency: `Collector` is `Clone` and can be shared across threads; internally uses `Arc<RwLock<...>>`.

Example: Concurrent recording across threads
```rust
use benchmark::{Collector, Duration};
use std::sync::Arc;
use std::thread;

let collector = Arc::new(Collector::new());
let mut handles = vec![];
for _ in 0..4 {
    let c = collector.clone();
    handles.push(thread::spawn(move || {
        for _ in 0..1000 {
            c.record_duration("io", Duration::from_nanos(100));
        }
    }));
}
for h in handles { h.join().unwrap(); }

let s = collector.stats("io").unwrap();
assert_eq!(s.count, 4 * 1000);
```

<br>

## Functions

### measure
Measures execution time of a closure and returns `(result, Duration)`.

```rust
use benchmark::measure;

let (result, duration) = measure(|| 2 + 2);
assert_eq!(result, 4);
```

- Enabled path: high-resolution timer via `std::time::Instant`.
- Disabled path (`!enabled`): returns `Duration::ZERO`.
- Overhead: designed to be minimal and competitive with direct `Instant` usage.

<br>

### measure_named
Measures execution time and returns `(result, Measurement)` with a name.

```rust
use benchmark::measure_named;

let (result, m) = measure_named("add", || 2 + 2);
assert_eq!(result, 4);
assert_eq!(m.name, "add");
```

- Timestamp set to UNIX epoch nanos (0 under Miri/isolation).
- Disabled path (`!enabled`): returns `Measurement { duration: ZERO, timestamp: 0 }`.

<br>

## Macros

### time!
Times an expression and returns `(result, Duration)`.

```rust
use benchmark::time;

let (result, dur) = time!(2 + 2);
assert_eq!(result, 4);
```

Async example (requires `features = ["std", "enabled"]`):
```rust
use benchmark::time;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let ((), d) = time!(tokio::time::sleep(std::time::Duration::from_millis(5)).await);
    assert!(d.as_millis() >= 5);
}
```

<br>

### time_named!
Times an expression with a name and returns `(result, Measurement)`.

```rust
use benchmark::time_named;

let (result, m) = time_named!("addition", 2 + 2);
assert_eq!(result, 4);
assert_eq!(m.name, "addition");
```

With `Collector`:
```rust
use benchmark::{time_named, Collector};

let collector = Collector::new();
let (_, m) = time_named!("db", {
    // your operation
    1 + 1
});
collector.record(&m);
let s = collector.stats("db").unwrap();
assert_eq!(s.count, 1);
```

Async example:
```rust
use benchmark::time_named;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let ((), m) = time_named!("sleep", tokio::time::sleep(std::time::Duration::from_millis(3)).await);
    assert!(m.duration.as_millis() >= 3);
}
```

Disabled example (`default-features = false`):
```rust
// returns Duration::ZERO/Measurement with zero duration
let (_out, d) = benchmark::time!(42);
assert_eq!(d.as_nanos(), 0);
```

<br>

## Async Usage
The macros inline timing using `std::time::Instant` under `features = ["std", "enabled"]` and fully support `await` inside the macro body. They can be used with any async runtime (Tokio, async-std, etc.).

Notes:
- When `enabled` is off, macros return zero durations but still evaluate expressions.
- Avoid holding locks across awaited code within your own operations.

<br>

## Disabled Mode Behavior
When compiled with `default-features = false` or without `enabled`:
- `measure()` returns `(result, Duration::ZERO)`.
- `measure_named()` returns `(result, Measurement { duration: ZERO, timestamp: 0 })`.
- `time!` returns `(result, Duration::ZERO)`.
- `time_named!` returns `(result, Measurement::zero(name))`.
- `Collector` and `Stats` are `std`-gated; if `std` is disabled they are not available.

<br>

<!--
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
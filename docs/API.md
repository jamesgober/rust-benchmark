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
- [Production Metrics (feature: metrics)](#production-metrics-feature-metrics)
  - [Watch](#watch)
  - [Timer](#timer)
  - [stopwatch!](#stopwatch)
- [Async Usage](#async-usage)
- [Disabled Mode Behavior](#disabled-mode-behavior)
  - [Best Practices: Handling 0ns in dashboards](#best-practices-handling-0ns-in-dashboards)
- [Examples](#examples)
  - [Rust Benchmark](#rust-benchmark)
  - [Code Benchmark](#code-benchmark)

<br><br>

## Installation

### Default Installation

#### Install Manually

Add this to your `Cargo.toml`:
```toml
[dependencies]
benchmark = "0.5.0"
```

<br>

#### Install via Terminal
```bash
# Basic installation (benchmarking feature only)
cargo add benchmark
```

<br>


###  Disable Default Features

#### Manually: Disable Default Features
Add this to your `Cargo.toml`:
```toml
[dependencies]
# Disable default features for true zero-overhead
benchmark = { version = "0.5.0", default-features = false }
```

<br>

#### Terminal: Disable Default Features
```bash
# Explicitly disabled - zero overhead
cargo add benchmark --no-default-features
```

<br>

&mdash; See [**`FEATURES DOCUMENTATION`**](./features/README.md) for more information.

<hr>
<br>





## Features
- `none` (optional): no features.
- `std` (default): uses Rust standard library; disables `no_std`
- `benchmark` (default): enables default benchmark measurement.
- `metrics` (optional): production/live metrics (`Watch`, `Timer`, `stopwatch!`).
- `default`: convenience feature equal to `std + benchmark`
- `standard`: convenience feature equal to `std + benchmark + metrics`
- `minimal`: minimal build with core timing only (*no default features*)
- `all`: Activates all features (*includes: `std + benchmark + metrics`*)

See [**`FEATURES DOCUMENTATION`**](./features/README.md) for more information.

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
- Disabled path (`!benchmark`): returns `Duration::ZERO`.
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
- Disabled path (`!benchmark`): returns `Measurement { duration: ZERO, timestamp: 0 }`.

<br>

## Macros

### time!
Times an expression and returns `(result, Duration)`.

```rust
use benchmark::time;

let (result, dur) = time!(2 + 2);
assert_eq!(result, 4);
```

Async example (requires `features = ["std", "benchmark"]`):
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

## Production Metrics (feature: metrics)
Provides production-friendly timing and percentile statistics with negligible overhead and zero cost when disabled.

Installation with feature:
```toml
[dependencies]
benchmark = { version = "0.5.0", features = ["standard"] }
```

### Watch
Thread-safe collector of nanosecond timings using a built-in, zero-dependency histogram under the hood.

```rust
use benchmark::Watch; // requires features = ["std", "metrics"]

let watch = Watch::new();
watch.record("db.query", 42_000);
let stats = watch.snapshot();
let s = &stats["db.query"];
assert!(s.p99 >= s.p50);
```

- Methods: `new()`, `builder() -> WatchBuilder`, `with_bounds(lowest, highest)`, `record(name, ns)`, `record_instant(name, start)`, `snapshot()`, `clear()`, `clear_name(name)`
- Concurrency: `Watch` is cheap to clone and `Send + Sync`.

### Timer
Records elapsed time to a `Watch` automatically when dropped.

```rust
use benchmark::{Timer, Watch};

let watch = Watch::new();
{
    let _t = Timer::new(watch.clone(), "render");
    // do work...
}
let s = watch.snapshot()["render"];
assert!(s.count >= 1);
```

### stopwatch!
Ergonomic macro to time a scoped block and record to a `Watch`. Works in sync and async contexts.

```rust
use benchmark::{stopwatch, Watch};

let watch = Watch::new();
stopwatch!(watch, "io", {
    std::thread::sleep(std::time::Duration::from_millis(1));
});
assert_eq!(watch.snapshot()["io"].count, 1);
```

Async example:
```rust
use benchmark::{stopwatch, Watch};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let watch = Watch::new();
    stopwatch!(watch, "sleep", {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    });
    assert_eq!(watch.snapshot()["sleep"].count, 1);
}
```

Notes:
- Percentiles are computed from histograms cloned outside locks for low contention.
- Durations are clamped to histogram bounds; defaults cover 1ns..~1h.
- Internal histogram: fixed-size, lock-free recording with nanosecond precision; zero external dependencies.

### Examples and Use-cases

The snippets below assume `features = ["standard"]`.

#### Real service loop (Tokio)
Record per-iteration latency and export periodically.

```rust
use benchmark::{stopwatch, Watch};

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let watch = Watch::new();

    // Periodic exporter (e.g., log or scrape endpoint)
    let exporter = {
        let w = watch.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                let snap = w.snapshot();
                for (name, s) in snap {
                    println!(
                        "metric={} count={} min={}ns p50={}ns p90={}ns p99={}ns max={}ns mean={:.1}",
                        name, s.count, s.min, s.p50, s.p90, s.p99, s.max, s.mean
                    );
                }
            }
        })
    };

    // Service work loop
    for i in 0..100u32 {
        stopwatch!(watch, "service.tick", {
            // do work (e.g., handle a batch)
            tokio::time::sleep(std::time::Duration::from_millis(5 + (i % 3) as u64)).await;
        });
    }

    exporter.abort();
}
```

#### Per-endpoint metrics (naming convention)
Use metric names to encode endpoint/method. Avoid user-provided strings directly.

```rust
use benchmark::{stopwatch, Watch};

fn endpoint_metric(method: &str, path: &str) -> String {
    // Prefer a stable, low-cardinality naming scheme
    format!("http.{}:{}", method, path) // e.g., http.GET:/users/:id
}

fn handle_request(watch: &Watch, method: &str, route_path: &str) {
    let name = endpoint_metric(method, route_path);
    stopwatch!(watch.clone(), &name, {
        std::thread::sleep(std::time::Duration::from_millis(2));
    });
}
```

#### Background worker
Measure I/O and processing separately with clear names.

```rust
use benchmark::{stopwatch, Watch};

let watch = Watch::new();
for _ in 0..1000 {
    stopwatch!(watch, "worker.fetch", {
        std::thread::sleep(std::time::Duration::from_millis(1));
    });
    stopwatch!(watch, "worker.process", {
        std::thread::sleep(std::time::Duration::from_millis(3));
    });
}
let s = watch.snapshot();
assert!(s["worker.process"].p90 >= s["worker.fetch"].p90);
```

#### Guard timer vs macro in async
- Prefer `stopwatch!` inside async code; it scopes correctly and is ergonomic.
- `Timer` also works, but be mindful of lifetimes and early `stop()` if you need to record before scope end.

```rust
use benchmark::{Timer, Watch};

let w = Watch::new();
let mut maybe_id = None;
let mut t = Timer::new(w.clone(), "job.run");
// ... compute an id
maybe_id = Some(42);
// early stop to record now
t.stop();
assert!(w.snapshot()["job.run"].count == 1);
```

#### Tuning histogram bounds
Set bounds to your SLOs to reduce memory and improve precision.

```rust
use benchmark::Watch;

// Builder: 100ns to 10s (fixed precision internally)
let watch = Watch::builder()
    .lowest(100)
    .highest(10_000_000_000)
    .build();
watch.record("op", 250);
```

#### Clearing/reset between intervals

```rust
use benchmark::Watch;

let w = Watch::new();
// ... record over a minute
let minute = w.snapshot();
// export minute
w.clear(); // start fresh for the next interval
```

#### Exporting snapshot
Iterate and serialize to your logging/metrics system. Below shows simple logging.

```rust
use benchmark::Watch;

fn export(w: &Watch) {
    for (name, s) in w.snapshot() {
        println!(
            "name={} count={} min={} p50={} p90={} p99={} max={} mean={:.2}",
            name, s.count, s.min, s.p50, s.p90, s.p99, s.max, s.mean
        );
    }
}
```

JSON export example (using serde_json):
```rust
// Add to Cargo.toml
// serde = { version = "1", features = ["derive"] }
// serde_json = "1"
use benchmark::Watch;
use serde::Serialize;

#[derive(Serialize)]
struct MetricRow<'a> {
    name: &'a str,
    count: u64,
    min: u64,
    p50: u64,
    p90: u64,
    p95: u64,
    p99: u64,
    p999: u64,
    max: u64,
    mean: f64,
}

fn export_json(w: &Watch) -> String {
    let mut rows = Vec::new();
    for (name, s) in w.snapshot() {
        rows.push(MetricRow {
            name: &name,
            count: s.count,
            min: s.min,
            p50: s.p50,
            p90: s.p90,
            p95: s.p95,
            p99: s.p99,
            p999: s.p999,
            max: s.max,
            mean: s.mean,
        });
    }
    serde_json::to_string_pretty(&rows).unwrap()
}
```

#### Contention tips
- Clone `Watch` freely and pass by value to tasks/threads.
- Use stable, low-cardinality metric names to keep the map small.
- If extremely hot, consider sharding names (e.g., per-core suffix) and merging snapshots offline.

## Async Usage
The macros inline timing using `std::time::Instant` under `features = ["std", "benchmark"]` and fully support `await` inside the macro body. They can be used with any async runtime (Tokio, async-std, etc.).

Notes:
- When `benchmark` is off, macros return zero durations but still evaluate expressions.
- Avoid holding locks across awaited code within your own operations.

<br>

## Disabled Mode Behavior
When compiled with `default-features = false` or without `benchmark`:
- `measure()` returns `(result, Duration::ZERO)`.
- `measure_named()` returns `(result, Measurement { duration: ZERO, timestamp: 0 })`.
- `time!` returns `(result, Duration::ZERO)`.
- `time_named!` returns `(result, Measurement::zero(name))`.
- `Collector` and `Stats` are `std`-gated; if `std` is disabled they are not available.

### Best Practices: Handling 0ns in dashboards
- Preserve fidelity in the data layer: zero durations are valid measurements for extremely fast operations.
- Apply a visualization floor at presentation time only if necessary (e.g., show 1ns instead of 0ns) to avoid skewing aggregates.
- Consider filtering 0ns when computing percentiles for SLO charts if they represent measurement granularity rather than business latency.
- If you need to avoid zeros in histograms, clamp on export, not at collection: `max(value, 1)`. Keep raw storage exact for audits.

<br>


<!--
EXAMPLES
######################################### -->
<br>
<div align="center">
  <a href="#top">&uarr;<br>TOP</a>
</div>
<hr>

<h2 id="examples">Examples</h2>


<h3 id="rust-benchmark">Rust Benchmark</h3>
{ADD_DESCRIPTION_HERE}

```rust
// Create a Rust benchmark similar 
// to Criterion,but using this library 
//instead of Criterion.
```

<br>

<h3 id="code-benchmark">Code Benchmark</h3>
{ADD_DESCRIPTION_HERE}

```rust
// Create a code (or code block) benchmark
// stress test using this library where 
// the benchmark test loops the code 
// block multiple times (as specified 
// in the benchmark function arguments).
```

<br>



<!-- END EXAMPLES -->
<br>
<hr>
<div align="center">
  <a href="#top">&uarr;<br>TOP</a>
</div>
<br>

<!--
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
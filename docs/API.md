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
<div align="center">
    <sup>
        <a href="../README.md" title="Project Home"><b>HOME</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./README.md" title="Project Documentation"><b>DOCS</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./features/README.md" title="Feature Flags"><b>FEATURES</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./BENCHMARK.md" title="Performance Benchmark"><b>BENCHMARK</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./METRICS.md" title="Performance Metrics"><b>METRICS</b></a>
    </sup>
</div>

<br>
> Note: Perf-sensitive tests/benches are skipped by default. To opt in, run with feature `perf-tests` and env `PERF_TESTS=1`. See "Performance Tests (opt-in)" below.

## Table of Contents
- [Installation](#installation)
- [Features](#features)
- [Types](#types)
  - [Duration](#duration)
  - [Measurement](#measurement)
  - [Stats](#stats)
  - [Histogram](#histogram)
- [Collector](#collector)
- [Functions](#functions)
  - [measure](#measure)
  - [measure_named](#measure_named)
- [Macros](#macros)
  - [time!](#time)
  - [time_named!](#time_named)
  - [benchmark_block!](#benchmark_block)
  - [benchmark!](#benchmark)
- [Production Metrics (feature: metrics)](#production-metrics-feature-metrics)
  - [Watch](#watch)
  - [Timer](#timer)
  - [stopwatch!](#stopwatch)
- [Async Usage](#async-usage)
- [Disabled Mode Behavior](#disabled-mode-behavior)
  - [Best Practices: Handling 0ns in dashboards](#best-practices-handling-0ns-in-dashboards)
- [Doctests and feature flags](#doctests-and-feature-flags)
- [Performance Tests (opt-in)](#performance-tests-opt-in)
- [Examples](#examples)
  - [Rust Benchmark](#rust-benchmark)
  - [Code Benchmark](#code-benchmark)
  - [Micro-Benchmarking](#micro-benchmarking)
  - [Macro-Benchmarking](#macro-benchmarking)
  - [A/B Testing](#ab-testing)
  - [Statistical Testing](#statistical-testing)
  - [Load Testing](#load-testing)
  - [Code Instrumentation](#code-instrumentation)
  - [Distributed Tracing](#distributed-tracing)
  - [Real-time Metrics](#real-time-metrics)
  - [Health Check Metrics](#health-check-metrics)
  - [APM Integration](#apm-integration)

<br><br>

## Installation

### Default Installation

#### Install Manually

Add this to your `Cargo.toml`:
```toml
[dependencies]
benchmark = "0.8.0"
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
benchmark = { version = "0.8.0", default-features = false }
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
- `benchmark` (default): timing functions and macros.
- `collector` (default): `Collector`, `Stats`, and built-in histogram backend.
- `metrics` (optional): production metrics (`Watch`, `Timer`, `stopwatch!`). Implies `collector`.
- `high-precision` (optional): enables high-precision histogram backend. Implies `collector`.
- `hdr` (optional): HDR histogram backend via optional `hdrhistogram` dependency. Requires `high-precision`. Initialization is non-panicking with a safe fallback in release builds.

Notes:
- `std` is internal and implied by the above features; you do not enable it directly.
- Minimal build: use `default-features = false` and selectively opt in.

&mdash; See [**`FEATURES DOCUMENTATION`**](./features/README.md) for more information.

<br>

## Quickstart
Minimal examples to get productive fast. See full examples below for more depth.

```rust
// Benchmarking (default features)
let (out, d) = benchmark::time!({ 2 + 2 });
assert_eq!(out, 4);
assert!(d.as_nanos() >= 0);

// Production metrics (features = ["metrics"]) 
let w = benchmark::Watch::new();
benchmark::stopwatch!(w, "op", { std::thread::sleep(std::time::Duration::from_millis(1)); });
assert!(w.snapshot()["op"].count >= 1);
```

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

### Histogram
Fixed-range, high-performance histogram used by production metrics. Available with `feature = "collector"` at `benchmark::histogram`.

```rust
use benchmark::histogram::Histogram; // requires feature = "collector"

let mut h = Histogram::new(1, 1_000_000); // bounds: 1ns..=1_000_000ns
for _ in 0..1000 { h.record(500); }
assert_eq!(h.count(), 1000);
assert_eq!(h.percentile(50.0), 500);
let ps = h.percentiles(&[50.0, 90.0, 99.0]);
assert!(ps[2] >= ps[0]);
```

- Constructors: `new(lowest: u64, highest: u64)`
- Recording: `record(ns: u64)`, `record_duration(Duration)`
- Queries: `count()`, `min()`, `max()`, `median()`, `percentile(q)`, `percentiles(&qs)`
- Notes: Inputs are clamped to bounds; percentile arguments `q` are clamped to [0.0, 1.0] (e.g., -0.1 -> 0.0, 1.2 -> 1.0); choose bounds to your expected SLOs for precision.

<br>

## Collector
Thread-safe aggregation of measurements. Available with `feature = "collector"`.

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

Async example (requires `feature = "benchmark"`):
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

With `Collector` (requires `features = ["benchmark", "collector"]`):
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

### benchmark_block!
Runs a code block repeatedly and returns raw per-iteration durations as `Vec<Duration>`.

Forms:

```rust
// Default iterations: 10_000
let samples: Vec<benchmark::Duration> = benchmark::benchmark_block!({
    // code to benchmark
    std::hint::black_box(1 + 1);
});

// Explicit iterations
let n = 1_234usize;
let samples = benchmark::benchmark_block!(n, {
    std::hint::black_box(2 * 3);
});
```

Notes:
- Raw data enables flexible downstream stats (mean, percentiles, etc.).
- Async-compatible: you can `await` inside the block.
- Disabled path (`!benchmark`): the block runs once, returns an empty vec for zero overhead.

Async example:
```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let samples = benchmark::benchmark_block!(100, {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    });
    assert_eq!(samples.len(), 100);
}
```

<br>

### benchmark!
Runs an expression repeatedly, labeling per-iteration samples. Returns `(Option<T>, Vec<Measurement>)` where `Option<T>` is the last result.

Forms:

```rust
// Default iterations: 10_000
let (last, samples) = benchmark::benchmark!("add", { 2 + 3 });

// Explicit iterations
let (last, samples) = benchmark::benchmark!("mul", 77usize, { 6 * 7 });
```

Notes:
- `samples[i].name == "add"` (or your provided name).
- Async-compatible: expressions/blocks may use `await`.
- Disabled path (`!benchmark`): runs once and returns `(Some(result), vec![])`.

Async example:
```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (_last, samples) = benchmark::benchmark!("sleep", 50usize, {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    });
    assert_eq!(samples.len(), 50);
}
```

<br>

## Production Metrics (feature: metrics)
Provides production-friendly timing and percentile statistics with negligible overhead and zero cost when disabled.

Installation with feature:
```toml
[dependencies]
benchmark = { version = "0.8.0", features = ["metrics"] }
```

### Watch
Thread-safe collector of nanosecond timings using a built-in, zero-dependency histogram under the hood.

```rust
use benchmark::Watch; // requires feature = "metrics"

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
use benchmark::{Timer, Watch}; // requires feature = "metrics"

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
use benchmark::{stopwatch, Watch}; // requires feature = "metrics"

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
- Percentile inputs are clamped to [0.0, 1.0]; out-of-range queries map to min/max.
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
use benchmark::Watch; // requires feature = "metrics"

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
use benchmark::Watch; // requires feature = "metrics"
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
The macros inline timing using `std::time::Instant` under `feature = "benchmark"` and fully support `await` inside the macro body. They can be used with any async runtime (Tokio, async-std, etc.).

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
- `benchmark_block!` executes once and returns `Vec::new()`.
- `benchmark!` executes once and returns `(Some(result), Vec::new())`.
- `Collector` and `Stats` are `collector`-gated; if `collector` is disabled they are not available.

## Common Pitfalls
- Ensure required features are enabled for copied snippets (`collector`, `metrics`).
- Zero durations are valid; avoid rewriting them at collection time. Clamp only at presentation.
- Tune histogram bounds near your SLOs for better percentile precision and lower memory.
- Keep metric names low-cardinality and stable to reduce map contention.
- Avoid holding your own locks across `await` inside timed regions.

### Best Practices: Handling 0ns in dashboards
- Preserve fidelity in the data layer: zero durations are valid measurements for extremely fast ops.
- Apply a visualization floor only at presentation time if necessary.
- Consider filtering 0ns when computing percentiles if they reflect timer granularity rather than business latency.
- If you must avoid zeros in histograms, clamp on export (`max(value, 1)`), not at collection.

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
Create a minimal Rust benchmark that repeatedly measures a function and reports summary statistics using `Collector`.

```rust
use benchmark::{Collector, time};

fn fibonacci(n: u64) -> u64 {
    match n { 0 => 0, 1 => 1, _ => fibonacci(n - 1) + fibonacci(n - 2) }
}

fn main() {
    let mut c = Collector::new();
    for _ in 0..1_000 {
        let (_, d) = time!(fibonacci(20));
        c.record_duration("fib20", d);
    }
    let s = c.stats("fib20").unwrap();
    println!("iterations={} mean={}ns min={}ns max={}ns",
        s.count, s.mean.as_nanos(), s.min.as_nanos(), s.max.as_nanos());
}
```

<br>

<h3 id="code-benchmark">Code Benchmark</h3>
Benchmark a code block by running it many times and collecting per-iteration durations using `benchmark_block!`.

```rust
use benchmark::benchmark_block;

fn hot() { std::hint::black_box(1 + 1); }

fn main() {
    // Default 10_000 iterations
    let samples = benchmark_block!({ hot() });
    assert_eq!(samples.len(), 10_000);

    // Explicit iterations
    let n = 5_000usize;
    let samples2 = benchmark_block!(n, { hot() });
    println!("n1={} n2={} first={}ns",
        samples.len(), samples2.len(), samples[0].as_nanos());
}
```

<br>

<h3 id="micro-benchmarking">Micro-Benchmarking</h3>
Measure small inner loops or tight functions; prefer deterministic inputs and avoid global state.

```rust
use benchmark::{Collector, time};

fn parse_u64(s: &str) -> u64 { s.parse().unwrap_or_default() }

fn main() {
    let mut c = Collector::new();
    for _ in 0..50_000 {
        let (_, d) = time!(parse_u64("123456"));
        c.record_duration("parse", d);
    }
    let s = c.stats("parse").unwrap();
    println!("count={} mean={}ns", s.count, s.mean.as_nanos());
}
```

<br>

<h3 id="macro-benchmarking">Macro-Benchmarking</h3>
Benchmark an end-to-end path (e.g., request handling). Capture realistic latencies across components.

```rust
use benchmark::{Collector, time};

fn handle_request() { std::thread::sleep(std::time::Duration::from_millis(3)); }

fn main() {
    let mut c = Collector::new();
    for _ in 0..1_000 { let (_, d) = time!(handle_request()); c.record_duration("req", d); }
    let s = c.stats("req").unwrap();
    println!("count={} min={}ns max={}ns mean={}ns",
        s.count, s.min.as_nanos(), s.max.as_nanos(), s.mean.as_nanos());
}
```

<br>

<h3 id="ab-benchmark">A/B Benchmark Testing</h3>
Compare multiple implementations by sampling each separately with identical workloads.

```rust
use benchmark::Collector;

fn impl_a(buf: &[u8]) -> usize { buf.iter().filter(|b| **b % 2 == 0).count() }
fn impl_b(buf: &[u8]) -> usize { buf.chunks(2).map(|c| c.len()).sum() }

fn main() {
    let data = vec![0u8; 4096];
    let mut ca = Collector::new();
    let mut cb = Collector::new();
    for _ in 0..10_000 { let (_, d) = benchmark::time!(impl_a(&data)); ca.record_duration("a", d); }
    for _ in 0..10_000 { let (_, d) = benchmark::time!(impl_b(&data)); cb.record_duration("b", d); }
    let sa = ca.stats("a").unwrap();
    let sb = cb.stats("b").unwrap();
    println!("A mean={}ns | B mean={}ns", sa.mean.as_nanos(), sb.mean.as_nanos());
}
```

<br>

<h3 id="statistical-testing">Benchmark: Statistical Testing</h3>
Sampling many iterations reduces noise and reveals distribution; compute summary stats.

```rust
use benchmark::Collector;

fn main() {
    let mut c = Collector::with_capacity(100_000);
    for _ in 0..100_000 { let (_, d) = benchmark::time!({ 1 + 1 }); c.record_duration("op", d); }
    let s = c.stats("op").unwrap();
    println!("n={} mean={}ns", s.count, s.mean.as_nanos());
}
```

<br>

<h3 id="load-testing">Load Testing</h3>
Generate sustained load to exercise systems and observe tail latency behavior.

```rust
use benchmark::Collector;

fn io() { std::thread::sleep(std::time::Duration::from_millis(1)); }

fn main() {
    let mut c = Collector::new();
    for _ in 0..5_000 { let (_, d) = benchmark::time!(io()); c.record_duration("io", d); }
    let s = c.stats("io").unwrap();
    println!("min={}ns p50~{}ns max={}ns", s.min.as_nanos(), s.mean.as_nanos(), s.max.as_nanos());
}
```

<br>

<h3 id="code-instrumentation">Code Instrumentation</h3>
Record production timings with minimal overhead using the `metrics` feature.

```rust
// Requires: features = ["std", "metrics"]
use benchmark::{stopwatch, Watch};

let watch = Watch::new();
stopwatch!(watch, "db.query", {
    std::thread::sleep(std::time::Duration::from_millis(2));
});
println!("count={}", watch.snapshot()["db.query"].count);
```

<br>


<h3 id="distributed-tracing">Distributed Tracing</h3>
Model spans for sub-operations (e.g., DB, cache, remote call) by naming timers consistently.

```rust
// Requires: features = ["std", "metrics"]
use benchmark::{stopwatch, Watch};

let w = Watch::new();
stopwatch!(w, "req.db", { std::thread::sleep(std::time::Duration::from_millis(1)); });
stopwatch!(w, "req.cache", { std::thread::sleep(std::time::Duration::from_millis(1)); });
stopwatch!(w, "req.http", { std::thread::sleep(std::time::Duration::from_millis(2)); });
```

<br>

<h3 id="real-time-metrics">Real-time Metrics</h3>
Continuously collect and snapshot percentiles with negligible overhead.

```rust
// Requires: features = ["std", "metrics"]
use benchmark::Watch;

let w = Watch::new();
for _ in 0..1000 { w.record("tick", 500); }
let s = w.snapshot()["tick"];
println!("p50={} p99={}", s.p50, s.p99);
```

<br>

<h3 id="health-check-metrics">Health Check Metrics</h3>
Track endpoint health like TTFB and response time; alert on SLO breaches.

```rust
// Requires: features = ["std", "metrics"]
use benchmark::{stopwatch, Watch};

let watch = Watch::new();
stopwatch!(watch, "health.ping", {
    std::thread::sleep(std::time::Duration::from_millis(1));
});
let s = watch.snapshot()["health.ping"]; 
println!("p99={}ns", s.p99);
```

<br>

<h3 id="apm-integration">APM Integration</h3>
Export snapshots to your logging/metrics stack periodically.

```rust
// Requires: features = ["std", "metrics"]
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

<br>

## Doctests and feature flags
Some examples require specific features to compile under doctest or when copy-pasted:

- `time!`, `measure`, `benchmark_block!`, `benchmark!`: Requires `feature = "benchmark"`.
- `Collector`, `Stats`, `histogram`: Requires `feature = "collector"`.
- `Watch`, `Timer`, `stopwatch!`: Requires `feature = "metrics"`.

When running doctests locally with docs.rs-like configuration, consider enabling all features:

```bash
RUSTDOCFLAGS="--cfg docsrs" cargo test --doc --all-features
```

Alternatively, gate your local snippets with cfgs when experimenting.

<br>

## Performance Tests (opt-in)
Perf-sensitive tests/benches are gated to avoid noisy CI variance. Opt in explicitly:

```bash
# run perf tests (ignored by default)
PERF_TESTS=1 cargo test -F perf-tests -- --ignored

# run benches that exercise perf paths
PERF_TESTS=1 cargo bench -F perf-tests
```

Notes:
- The `perf-tests` feature gates perf-sensitive code in tests/benches.
- Tests also check `PERF_TESTS` at runtime and will early-exit when not set.


<!--
:: END EXAMPLES
============================================================================ -->
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
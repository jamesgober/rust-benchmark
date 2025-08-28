<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br><b>Benchmark</b><br>
    <sub><sup>
        BENCHMARK FEATURE
    </sup></sub>
</h1>
<div align="center">
    <sup>
    <a href="../../README.md" title="Project Home"><b>HOME</b></a>
    <span>&nbsp;│&nbsp;</span>
    <a href="../README.md" title="Project Documentation"><b>DOCS</b></a>
    <span>&nbsp;│&nbsp;</span>
    <a href="../API.md" title="API Reference"><b>API</b></a>
    <span>&nbsp;│&nbsp;</span>
    <a href="./README.md" title="Feature Flags"><b>FEATURES</b></a>
    </sup>
</div>

<br>
<br>

<p>
The <b>benchmark</b> feature provides a lightweight, statistically-sound toolkit for development-time performance testing. It includes timing helpers, collectors, and summary statistics designed to detect regressions, compare implementations, and measure realistic variability across iterations.
</p>

<br>

## Purpose
- Overall performance testing of a crate or application
- Targeted testing of a function, library, service, or protocol
- Detecting performance regressions in CI
- Comparative analysis across multiple implementations
- Stress-testing to understand tail latency behavior

<br>

🧩 **API**: 
[**`Measurement`**](../API.md#measurement),
[**`Stats`**](../API.md#stats),
[**`Collector`**](../API.md#collector),
[**`measure`**](../API.md#measure),
[**`measure_named`**](../API.md#measure_named),
[**`time`**](../API.md#time),
[**`time_named`**](../API.md#time_named) 

<br>

## Installation


### Manual installation:
```toml
[dependencies]

# Benchmark is enabled by default.
benchmark = "0.5.8"

# or enable benchmark directly. 
benchmark = { version = "0.5.8", features = ["benchmark", "std"]}
```
> ⚙️ Add directly to your `Cargo.toml`.

<br>

### Installation via terminal:
```bash
# Benchmark is enabled by default.
cargo add benchmark

# or enable benchmark directly. 
cargo add benchmark -F benchmark -F std
```
> ⚙️ Using the `cargo add` command.

<br>
<hr>
<br>

## Examples


### Basic benchmark test
```rust
use benchmark::{Collector};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    // Run the code multiple times and collect a Stats summary
    let mut c = Collector::new();
    for _ in 0..1_000 {
        let (_, d) = benchmark::time!(fibonacci(20));
        c.record_duration("fib20", d);
    }
    let s = c.stats("fib20").unwrap();
    println!("iterations={} mean={}ns min={}ns max={}ns",
        s.count, s.mean.as_nanos(), s.min.as_nanos(), s.max.as_nanos());
}
```


### Micro-Benchmarking with `benchmark_block!`
Collect raw per-iteration durations for tight loops or inner functions.
```rust
use benchmark::benchmark_block;

fn hot() { std::hint::black_box(1 + 1); }

fn main() {
    // Default 10_000 iterations
    let samples = benchmark_block!({ hot() });
    assert_eq!(samples.len(), 10_000);

    // Explicit iterations
    let samples = benchmark_block!(5_000usize, { hot() });
    println!("n={} first={}ns", samples.len(), samples[0].as_nanos());
}
```

<br>

### Macro-Benchmarking with `benchmark!`
Name your benchmark and capture the last result plus labeled measurements.
```rust
use benchmark::benchmark;

fn parse(input: &str) -> i64 { input.parse().unwrap_or_default() }

fn main() {
    // Default 10_000 iterations
    let (last, ms) = benchmark!("parse", { parse("12345") });
    assert_eq!(last, Some(12345));
    assert_eq!(ms[0].name, "parse");

    // Explicit iterations
    let (_last, ms2) = benchmark!("double", 500usize, { 2 * 2 });
    assert_eq!(ms2.len(), 500);
}
```

<small>
Disabled mode (`default-features = false`): `benchmark_block!` runs once and returns `vec![]`; `benchmark!` runs once and returns `(Some(result), vec![])`.
</small>


### Standard bench test (named measurements)
```rust
use benchmark::{Collector};

fn parse_int(input: &str) -> i64 { input.parse::<i64>().unwrap_or_default() }

fn main() {
    let mut c = Collector::new();
    for _ in 0..10_000 {
        let (_, m) = benchmark::time_named!("parse", parse_int("12345"));
        c.record(&m);
    }
    let s = c.stats("parse").unwrap();
    println!("count={} mean={}ns", s.count, s.mean.as_nanos());
}
```


### Benchmark Comparison
```rust
use benchmark::Collector;

fn impl_a(buf: &[u8]) -> usize { buf.iter().filter(|b| **b % 2 == 0).count() }
fn impl_b(buf: &[u8]) -> usize { buf.chunks(2).map(|c| c.len()).sum() }

fn main() {
    let data = vec![0u8; 4096];

    let mut a = Collector::new();
    for _ in 0..10_000 { let (_, d) = benchmark::time!(impl_a(&data)); a.record_duration("a", d); }
    let sa = a.stats("a").unwrap();

    let mut b = Collector::new();
    for _ in 0..10_000 { let (_, d) = benchmark::time!(impl_b(&data)); b.record_duration("b", d); }
    let sb = b.stats("b").unwrap();

    println!("impl_a mean={}ns", sa.mean.as_nanos());
    println!("impl_b mean={}ns", sb.mean.as_nanos());
}
```


### Bench with custom sampling
```rust
use benchmark::Collector;

fn work() { std::hint::black_box(1 + 1); }

fn main() {
    let mut c = Collector::with_capacity(5_000);
    for _ in 0..5_000 {
        let (_, d) = benchmark::time!(work());
        c.record_duration("work", d);
    }
    let s = c.stats("work").unwrap();
    println!("count={} min={}ns max={}ns", s.count, s.min.as_nanos(), s.max.as_nanos());
}
```





<br>

## How to run perf benchmarks (Criterion)
Perf-sensitive benches are disabled by default. Enable explicitly to avoid noisy CI by default and to run only when desired.

```bash
# Run all perf-gated benches
PERF_TESTS=1 cargo bench -F perf-tests

# Run a specific bench target
PERF_TESTS=1 cargo bench -F perf-tests timers
PERF_TESTS=1 cargo bench -F perf-tests histogram_hot
PERF_TESTS=1 cargo bench -F "perf-tests metrics" watch_timer_hot
```

Bench files and groups:
- `benches/timers.rs` → group: `timers`
  - Measures `Instant::now()` throughput and `Duration` arithmetic
- `benches/histogram_hot.rs` → group: `histogram_hot`
  - Measures `Histogram::record` and `Histogram::percentiles`
- `benches/watch_timer_hot.rs` → group: `watch_timer_hot`
  - Measures `Watch::record`, `Watch::record_instant`, `Watch::snapshot` scaling, and `Timer` drop-throughput

Notes:
- CI scheduled perf runs are configured in `.github/workflows/perf.yml`.
- When `perf-tests` is disabled, benches build with a no-op `main()` to prevent linkage errors.

<!--
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
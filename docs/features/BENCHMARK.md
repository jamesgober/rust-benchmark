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
benchmark = "0.5.0"

# or enable benchmark directly. 
benchmark = { version = "0.5.0", features = ["benchmark", "std"]}
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

<!--
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
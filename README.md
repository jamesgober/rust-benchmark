<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>Benchmark</b>
    <br>
    <sub><sup>RUST LIBRARY</sup></sub>
</h1>
<div align="center">
    <a href="https://crates.io/crates/benchmark"><img alt="Crates.io" src="https://img.shields.io/crates/v/benchmark"></a>
    <a href="https://crates.io/crates/benchmark" alt="Download benchmark"><img alt="Crates.io Downloads" src="https://img.shields.io/crates/d/benchmark?color=%230099ff"></a>
    <a href="https://docs.rs/benchmark" title="Benchmark Documentation"><img alt="docs.rs" src="https://img.shields.io/docsrs/benchmark"></a>
    <a href="https://github.com/jamesgober/rust-benchmark/actions"><img alt="GitHub CI" src="https://github.com/jamesgober/rust-benchmark/actions/workflows/ci.yml/badge.svg"></a>
    <a href="https://github.com/jamesgober/rust-benchmark/actions/workflows/bench.yml" title="Benchmarks Workflow"><img alt="Benchmarks" src="https://github.com/jamesgober/rust-benchmark/actions/workflows/bench.yml/badge.svg"></a>
    <a href="https://github.com/rust-lang/rfcs/blob/master/text/2495-min-rust-version.md" title="MSRV"><img alt="MSRV" src="https://img.shields.io/badge/MSRV-1.70%2B-blue"></a>
</div>
<br>
<p>
    Nanosecond-precision benchmarking for <b>development</b>, <b>testing</b>, and <b>production</b>.
    The <b>core timing path</b> is <b>zero-overhead</b> when disabled, while optional, std-powered
    <b>collectors</b> and <b>zero-dependency metrics</b> (<code>Watch</code>/<code>Timer</code> and the
    <code>stopwatch!</code> macro) provide real <b>service observability</b> with percentiles in production.
    Designed to be embedded in performance-critical code without bloat or footguns.
  </p>


<br>

<h2>What is Benchmark?</h2>
<p>
<strong>Benchmark</strong> is a comprehensive benchmarking suite with an advanced performance metrics module that functions in two distinct contexts:

<h3>1: As a Development Tool:</h3>
<p>
  The <strong>Benchmarking Suite</strong> is a <b>statistical benchmarking framework</b> for performance testing and optimization during development. This suite provides the tools for <b>statistical microbenchmarking and comparative analysis</b>, allowing you to <b>detect performance regressions in CI</b> and <b>make data-driven implementation choices during development</b>.
</p>

<h4>Benchmarking Suite Features</h4>
<p>A lightweight, ergonomic alternative to Criterion for statistical performance testing:</p>
<ul>
    <li><a href="./docs/BENCHMARK.md#micro-benchmarking"><b>Micro-Benchmarking</b></a>: Test isolated code blocks with the <code>benchmark_block!</code> macro.</li>
    <li><a href="./docs/BENCHMARK.md#macro-benchmarking"><b>Macro-Benchmarking</b></a>: Test entire functions or modules with the <code>benchmark!</code> macro.</li>
    <li><a href="./docs/BENCHMARK.md#comparative-analysis"><b>Comparative Analysis</b></a>: A/B test multiple implementations to make data-driven optimization decisions.</li>
    <li><a href="./docs/BENCHMARK.md#statistical-sampling"><b>Statistical Sampling</b></a>: Runs code repeatedly to generate robust performance statistics.</li>
    <li><a href="./docs/BENCHMARK.md#ci-cd-integration"><b>CI/CD Integration</b></a>: Detect performance regressions in your deployment pipeline.</li>
    <li><a href="./docs/BENCHMARK.md#load-testing"><b>Load Testing</b></a>: Simulate realistic traffic patterns and stress test your code.</li>
</ul>
<p>
  &mdash; See <a href="./docs/BENCHMARK.md"><b>Benchmark Documentation</b></a> for more information.
</p>

<br>

<h3>2: As a Production Tool:</h3>
<p>
  The <strong>performance metrics</strong> module serves as an advanced, lightweight <b>application performance monitoring (APM)</b> tool that provides seemless production observability. It allows you to instrument your application to capture <b>real-time performance metrics</b> for critical operations like database queries and API response times. Each timing measurement can be thought of as a span, helping you identify bottlenecks in a live system.
</p>
<h4>Performance Metrics Features</h4>
<p>Low-overhead instrumentation for live application monitoring:</p>
<ul>
    <li><a href="./docs/METRICS.md#code-instrumentation"><b>Code Instrumentation</b></a>: Add performance timers to production code with minimal overhead!</li>
    <li><a href="./docs/METRICS.md#distributed-tracing"><b>Distributed Tracing</b></a>: Break down complex operations into spans (database queries, API calls, etc.).</li>
    <li><a href="./docs/METRICS.md#real-time-metrics"><b>Real-time Metrics</b></a>: Capture every operation's performance data, not statistical averages.</li>  
    <li><a href="./docs/METRICS.md#health-check-metrics"><b>Health Check Metrics</b></a>: Monitor TTFB, response times, and system health.</li>
    <li><a href="./docs/METRICS.md#apm-integration"><b>APM Integration</b></a>: Core observability tooling for application performance monitoring.</li>
</ul>
<p>
   &mdash; See <a href="./docs/METRICS.md"><b>Metrics Documentation</b></a> for more information.
</p>

<hr>
<br>

<h2>Features</h2>
<ul>
    <li><b>True Zero-Overhead:</b> When disabled via <code>default-features = false</code>, all benchmarking code compiles away completely, adding zero bytes to your binary and zero nanoseconds to execution time.</li>
    <li><b>No Dependencies:</b> Built using only the <b>Rust standard library</b>, eliminating dependency conflicts and keeping compilation times fast.</li>
    <li><b>Thread-Safe by Design:</b> Core measurement functions are pure and inherently thread-safe, with an optional thread-safe Collector for aggregating measurements across threads.</li>
    <li><b>Async Compatible:</b> Works seamlessly with any async runtime (<code>Tokio</code>, <code>async-std</code>, etc.) without special support or additional features - just time any expression, sync or async.</li>
    <li><b>Nanosecond Precision:</b> Uses platform-specific high-resolution timers through <code>std::time::Instant</code>, providing nanosecond-precision measurements with monotonic guarantees.</li>
    <li><b>Simple Statistics:</b> Provides essential statistics (<code>count</code>, <code>total</code>, <code>min</code>, <code>max</code>, <code>mean</code>) without complex algorithms or memory allocation, keeping the library focused and efficient.</li>
    <li><b>Production Metrics (optional):</b> Enable the <code>metrics</code> feature for a thread-safe <code>Watch</code>, <code>Timer</code> (auto record on drop), and <code>stopwatch!</code> macro using a <b>built-in zero-dependency histogram</b> for percentiles.</li>
    <li><b>Minimal API Surface:</b> Just four functions and two macros - easy to learn, hard to misuse, and unlikely to ever need breaking changes.</li>
    <li><b>Cross-Platform:</b> Consistent behavior across <b>Linux</b>, <b>macOS</b>, <b>Windows</b>, and other platforms supported by Rust's standard library.</li>
</ul>

<hr>
<br>

## Feature Flags
- `none`: no features.
- `std` (*default*): uses Rust standard library; disables `no_std`
- `benchmark` (*default*): enables default benchmark measurement.
- `metrics` (*optional*): production/live metrics (`Watch`, `Timer`, `stopwatch!`).
- `default`: convenience feature equal to `std + benchmark`
- `standard`: convenience feature equal to `std + benchmark + metrics`
- `minimal`: minimal build with core timing only (*no default features*)
- `all`: Activates all features (*includes: `std + benchmark + metrics`*)

&mdash; See [**`FEATURES DOCUMENTATION`**](./docs/features/README.md) for more information.

<br>
<hr>
<br>

<h2>Usage:</h2>

<br>

### Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
benchmark = "0.7.2"
```

<br>


### Standard Features
> Enables all standard benchmark features.
```toml
[dependencies]

# Enables Production & Development.
benchmark = { version = "0.7.2", features = ["standard"] }
```

<br>

### Production metrics (std + metrics)
Enable production observability using `Watch`/`Timer` or the `stopwatch!` macro.

Cargo features:
```toml
[dependencies]
benchmark = { version = "0.7.2", features = ["std", "metrics"] }
```

Record with `Timer` (auto-record on drop):
```rust
use benchmark::{Watch, Timer};

let watch = Watch::new();
{
    let _t = Timer::new(watch.clone(), "db.query");
    // ... do the work to be measured ...
} // recorded once on drop

let s = &watch.snapshot()["db.query"];
assert!(s.count >= 1);
```

Or use the `stopwatch!` macro:
```rust
use benchmark::{Watch, stopwatch};

let watch = Watch::new();
stopwatch!(watch, "render", {
    // ... work to measure ...
});
assert!(watch.snapshot()["render"].count >= 1);
```

<br>

### Disable Default Features
> True zero-overhead core timing only.
```toml
[dependencies]
# Disable default features for true zero-overhead
benchmark = { version = "0.7.2", default-features = false }
```
<br>

&mdash; See [**`FEATURES DOCUMENTATION`**](./docs/features/README.md) for more information.

<hr>
<br>

## Quick Start


<small>
See also: <a href="./docs/API.md#async-usage"><b>Async Usage</b></a> · <a href="./docs/API.md#disabled-mode-behavior"><b>Disabled Mode Behavior</b></a> · <a href="./docs/API.md#production-metrics-feature-metrics"><b>Production Metrics</b></a>
</small>

### Measure a closure
Use `measure()` to time any closure and get back `(result, Duration)`.
```rust
use benchmark::measure;

let (value, duration) = measure(|| 2 + 2);
assert_eq!(value, 4);
println!("took {} ns", duration.as_nanos());
```

<br>

### Time an expression with the macro
`time!` works with any expression and supports async contexts.
```rust
use benchmark::time;

let (value, duration) = time!({
    let mut sum = 0;
    for i in 0..10_000 { sum += i; }
    sum
});
assert!(duration.as_nanos() > 0);
```

### Micro-benchmark a code block
Use `benchmark_block!` to run a block many times and get raw per-iteration durations.
```rust
use benchmark::benchmark_block;

// Default 10_000 iterations
let samples = benchmark_block!({
    // hot path
    std::hint::black_box(1 + 1);
});
assert_eq!(samples.len(), 10_000);

// Explicit iterations
let samples = benchmark_block!(1_000usize, {
    std::hint::black_box(2 * 3);
});
```

<br>

### Macro-benchmark a named expression
Use `benchmark!` to run a named expression repeatedly and get `(last, Vec<Measurement>)`.
```rust
use benchmark::benchmark;

// Default 10_000 iterations
let (last, ms) = benchmark!("add", { 2 + 3 });
assert_eq!(last, Some(5));
assert_eq!(ms[0].name, "add");

// Explicit iterations
let (_last, ms) = benchmark!("mul", 77usize, { 6 * 7 });
assert_eq!(ms.len(), 77);
```

<small>
Disabled mode (`default-features = false`): `benchmark_block!` runs once and returns `vec![]`; `benchmark!` runs once and returns `(Some(result), vec![])`.
</small>

<br>

### Named timing + Collector aggregation (std + benchmark)
Record a named measurement and aggregate stats with `Collector`.
```rust
use benchmark::{time_named, Collector};

fn work() {
    std::thread::sleep(std::time::Duration::from_millis(1));
}

let collector = Collector::new();
let (_, m) = time_named!("work", work());
collector.record(&m);

let stats = collector.stats("work").unwrap();
println!(
    "count={} total={}ns min={}ns max={}ns mean={}ns",
    stats.count,
    stats.total.as_nanos(),
    stats.min.as_nanos(),
    stats.max.as_nanos(),
    stats.mean.as_nanos()
);
```

<br>

### Async timing with `await`
The macros inline `Instant` timing when `features = ["std", "benchmark"]`, so awaiting inside works seamlessly.
```rust
use benchmark::time;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let sleep_dur = std::time::Duration::from_millis(10);
    let ((), d) = time!(tokio::time::sleep(sleep_dur).await);
    println!("slept ~{} ms", d.as_millis());
}
```

<hr>

<h2>Benchmarks</h2>
<p>
This repository includes Criterion benchmarks that measure the overhead of the public API compared to a direct <code>Instant::now()</code> baseline.
</p>

<h3>Performance Baselines &amp; CI</h3>
<p>
Baseline comparison is integrated into the <code>Perf</code> workflow (<code>.github/workflows/perf.yml</code>) and summarizes results in the GitHub <b>Step Summary</b>.
The comparator script <code>scripts/compare_criterion_baseline.sh</code> checks current Criterion outputs against JSON baselines in <code>perf_baselines/*.json</code> with per-key tolerances.
</p>
<ul>
  <li><b>Lenient by default</b>: comparisons run with <code>PERF_COMPARE_STRICT=0</code> so regressions are <b>reported but CI passes</b>.</li>
  <li><b>Strict gate (optional)</b>: set <code>PERF_COMPARE_STRICT=1</code> on a step to make regressions <b>fail</b> the job.</li>
  <li><b>Layouts supported</b>: both nested (<code>target/criterion/&lt;group&gt;/.../new/estimates.json</code>) and flat (<code>target/criterion/.../new/estimates.json</code>).</li>
  <li><b>Local run</b>:
    <pre><code>cargo bench -F perf-tests --bench watch_timer_hot -- --measurement-time 5 --warm-up-time 2 --save-baseline current
bash scripts/compare_criterion_baseline.sh watch_timer_hot perf_baselines/watch_timer_hot.json</code></pre>
  </li>
</ul>

<hr>

<h2>Safety &amp; Edge Cases</h2>
<ul>
  <li><b>Zero durations</b>: Some operations can complete so fast that <code>elapsed()</code> may be 0ns on some platforms. The API preserves this and returns <code>Duration::ZERO</code>. If you need a floor for visualization, clamp in your presentation layer.</li>
  <li><b>Saturating conversions</b>: <code>Watch::record_instant()</code> converts <code>as_nanos()</code> to <code>u64</code> using saturating semantics, avoiding panics on extremely large values.</li>
  <li><b>Range clamping</b>: <code>Watch::record()</code> clamps input to histogram bounds, ensuring valid recording and protecting against out-of-range values.</li>
  <li><b>Percentile input clamping</b>: Percentile queries (e.g., <code>Histogram::percentile(q)</code>) clamp <code>q</code> to <b>[0.0, 1.0]</b>. Out-of-range inputs map to min/max (e.g., <code>-0.1 → 0.0</code>, <code>1.2 → 1.0</code>).</li>
  <li><b>Drop safety</b>: <code>Timer</code> records exactly once. It guards against double-record by storing <code>Option&lt;Instant&gt;</code> and recording on <code>Drop</code> even during unwinding.</li>
  <li><b>Empty datasets</b>: <code>Watch::snapshot()</code> and <code>Collector</code> handle empty sets defensively. Snapshots for empty histograms return zeros; <code>Collector::stats()</code> returns <code>None</code> for missing keys.</li>
  <li><b>Overflow protection</b>: <code>Collector</code> uses <code>saturating_add</code> for total accumulation and 128-bit nanosecond storage in <code>Duration</code> to provide ample headroom.</li>
  <li><b>Thread safety</b>: All shared structures use <code>RwLock</code> with short hold-times: clone under read lock, compute outside the lock. Methods <b>recover from lock poisoning</b> (e.g., <code>unwrap_or_else(|e| e.into_inner())</code>) to avoid panics in production.</li>
  <li><b>Feature gating</b>: Production metrics are gated behind <code>features=["std","metrics"]</code>. Disable default features to make all timing a no-op for zero-overhead builds.</li>
</ul>

<h3>How to run</h3>
<pre><code>cargo bench
</code></pre>

<hr>

<h2>Zero-overhead Proof</h2>
<p>
This project includes automated checks that verify zero bytes and zero time when disabled, and provide assembly and runtime comparison artifacts.
</p>

<ul>
  <li><b>Size comparison (CI)</b>: Workflow <code>.github/workflows/ci.yml</code>, job <b>Zero Overhead</b> builds the crate twice and compares <code>libbenchmark.rlib</code> sizes with and without <code>benchmark</code>. Any unexpected growth fails the job.</li>
  <li><b>Assembly artifacts (CI)</b>: Job <b>Assembly Inspection</b> emits <code>objdump</code>/<code>llvm-objdump</code> symbol tables and disassembly for enabled vs disabled. Download the <b>assembly-artifacts</b> artifact from the run to inspect.</li>
  <li><b>Runtime comparison (CI)</b>: Bench workflow <code>.github/workflows/bench.yml</code> runs the example <code>examples/overhead_compare.rs</code> in both modes and uploads <b>overhead-compare</b> artifacts with raw output.</li>
  <li><b>Compile tests (trybuild)</b>: <code>tests/trybuild_disabled.rs</code> ensures disabled mode compiles with the macro and function APIs used in a minimal program.</li>
  <li><b>Local reproduction</b>:
    <ul>
      <li>Disabled: <code>cargo run --release --example overhead_compare --no-default-features</code></li>
      <li>Benchmark: <code>cargo run --release --example overhead_compare --no-default-features --features "std benchmark"</code></li>
    </ul>
  </li>
</ul>

<p>
When features are disabled (<code>default-features = false</code>), timing returns <code>Duration::ZERO</code> and produces no runtime cost (e.g., <code>time_macro_ns=0</code> on CI-hosted runners), validating the zero-overhead guarantee.
</p>

<h3>Sample results (illustrative)</h3>
<p><i>Results below are from a recent run on GitHub-hosted Linux runners; your numbers will vary by hardware and load.</i></p>
<pre><code>Overhead
--------
instant_now_elapsed           ~ 81–89 ns
measure_closure_add           ~ 79–81 ns
time_macro_add                ~ 81–82 ns

Collector Statistics
--------------------
stats::single/1000            ~ 2.44–2.61 µs
stats::single/10000           ~ 26.7–29.1 µs
stats::all/k10_n1000          ~ 29.4–33.6 µs
stats::all/k50_n1000          ~ 148.6–157.1 µs

Array Baseline (no locks)
-------------------------
stats::array/k1_n10000        ~ 17.15–17.90 µs
stats::array/k10_n1000        ~ 15.03–16.25 µs
</code></pre>

<h4>Interpretation</h4>
<ul>
  <li><b>Macro/function overhead</b> is on par with direct <code>Instant</code> usage for trivial work, as expected.</li>
  <li><b>Collector stats</b> scale roughly linearly with the number of samples; costs are dominated by iteration and min/max/accumulate.</li>
  <li><b>No-lock array baseline</b> provides a lower bound for aggregation cost; the difference vs Collector indicates lock and map overhead.</li>
  <li>Use <code>--features std,benchmark</code> to ensure the benchmark timing path is used.</li>
</ul>

<h3>Benchmarks included</h3>
<ul>
  <li><b>overhead::instant</b>: <code>Instant::now().elapsed()</code> baseline.</li>
  <li><b>overhead::measure</b>: <code>measure(|| expr)</code>.</li>
  <li><b>overhead::time_macro</b>: <code>time!(expr)</code>.</li>
  <li>Bench source: <code>benches/overhead.rs</code>.</li>
  <li>Criterion version: <code>0.5</code>.</li>
  <li>Note: when features are disabled (<code>default-features = false</code>), measurement returns <code>Duration::ZERO</code>.</li>
  <li>Tip: use <code>--features std,benchmark</code> to ensure the benchmark path for benches.</li>
  <li>Environment affects results; run on a quiet system with performance governor if possible.</li>
</ul>

<h3>Sample output (placeholder)</h3>
<pre><code>overhead::instant/instant_now_elapsed
                        time:   [X ns  ..  Y ns  ..  Z ns]

overhead::measure/measure_closure_add
                        time:   [X ns  ..  Y ns  ..  Z ns]

overhead::time_macro/time_macro_add
                        time:   [X ns  ..  Y ns  ..  Z ns]
</code></pre>


<!-- API REFERENCE
############################################# -->
<hr>

<h3>Documentation:</h3>
<ul>
    <li><a href="./docs/API.md"><b>API Reference</b></a> Complete documentation and examples.</li>
    <li><a href="./docs/API.md#production-metrics-feature-metrics"><b>Production Metrics</b></a> <code>Watch</code>, <code>Timer</code>, and <code>stopwatch!</code>.</li>
    <li><a href="./docs/PRINCIPLES.md"><b>Code Principles</b></a> guidelines for contribution &amp; development.</li>
</ul>

<hr>

<h2>MSRV &amp; SemVer Policy</h2>
<ul>
  <li><b>MSRV</b>: 1.70+ (as indicated by the badge). We bump MSRV only in <b>minor</b> releases and document it in the changelog.</li>
  <li><b>SemVer</b>:
    <ul>
      <li>Patch (x.y.<b>z</b>): bug fixes, internal improvements, and documentation.</li>
      <li>Minor (x.<b>y</b>.z): new features and non-breaking API additions.</li>
      <li>Major (<b>x</b>.y.z): breaking changes only, announced in the changelog with migration notes.</li>
    </ul>
  </li>
  <li><b>Feature flags</b>: additive and stable. Disabled paths remain zero-cost.</li>
  <li><b>No unsafe</b> in hot paths; any future "unsafe" will be justified and tested.</li>
  <li><b>Docs.rs</b>: examples note required feature flags to avoid confusion.</li>
  </ul>

<hr>

<h2 align="center">
    DEVELOPMENT &amp; CONTRIBUTION
</h2>

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

<hr>
<br>

<!-- LICENSE
############################################# -->
<div id="license">
    <h2>⚖️ License</h2>
    <p>Licensed under the <b>Apache License</b>, version 2.0 (the <b>"License"</b>); you may not use this software, including, but not limited to the source code, media files, ideas, techniques, or any other associated property or concept belonging to, associated with, or otherwise packaged with this software except in compliance with the <b>License</b>.</p>
    <p>You may obtain a copy of the <b>License</b> at: <a href="http://www.apache.org/licenses/LICENSE-2.0" title="Apache-2.0 License" target="_blank">http://www.apache.org/licenses/LICENSE-2.0</a>.</p>
    <p>Unless required by applicable law or agreed to in writing, software distributed under the <b>License</b> is distributed on an "<b>AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND</b>, either express or implied.</p>
    <p>See the <a href="./LICENSE" title="Software License file">LICENSE</a> file included with this project for the specific language governing permissions and limitations under the <b>License</b>.</p>
</div>

<br>

<!-- COPYRIGHT
############################################# -->
<div align="center">
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
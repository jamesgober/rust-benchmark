<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>Benchmark</b>
    <br>
    <sub>
        <sup>RUST LIBRARY</sup>
    </sub>
    <br>
</h1>
<div align="center">
    <a href="https://crates.io/crates/benchmark"><img alt="Crates.io" src="https://img.shields.io/crates/v/benchmark"></a>
    <span>&nbsp;</span>
    <a href="https://crates.io/crates/benchmark" alt="Download benchmark"><img alt="Crates.io Downloads" src="https://img.shields.io/crates/d/benchmark?color=%230099ff"></a>
    <span>&nbsp;</span>
    <a href="https://docs.rs/benchmark" title="Benchmark Documentation"><img alt="docs.rs" src="https://img.shields.io/docsrs/benchmark"></a>
    <span>&nbsp;</span>
    <a href="https://github.com/jamesgober/rust-benchmark/actions"><img alt="GitHub CI" src="https://github.com/jamesgober/rust-benchmark/actions/workflows/ci.yml/badge.svg"></a>
    <span>&nbsp;</span>
    <a href="https://github.com/jamesgober/rust-benchmark/actions/workflows/bench.yml" title="Benchmarks Workflow"><img alt="Benchmarks" src="https://github.com/jamesgober/rust-benchmark/actions/workflows/bench.yml/badge.svg"></a>
</div>

<p>
    Nanosecond-precision benchmarking for <b>development</b>, <b>testing</b>, and <b>production</b>.
    The <b>core timing path</b> is <b>zero-overhead</b> when disabled, while optional, std-powered
    <b>collectors</b> and <b>hdrhistogram-based metrics</b> (<code>Watch</code>/<code>Timer</code> and the
    <code>stopwatch!</code> macro) provide real <b>service observability</b> with percentiles in production.
    Designed to be embedded in performance-critical code without bloat or footguns.
  </p>

<br>

<h2>Features</h2>
<ul>
    <li><b>True Zero-Overhead:</b> When disabled via <code>default-features = false</code>, all benchmarking code compiles away completely, adding zero bytes to your binary and zero nanoseconds to execution time.</li>
    <li><b>No Dependencies:</b> Built using only the <b>Rust standard library</b>, eliminating dependency conflicts and keeping compilation times fast.</li>
    <li><b>Thread-Safe by Design:</b> Core measurement functions are pure and inherently thread-safe, with an optional thread-safe Collector for aggregating measurements across threads.</li>
    <li><b>Async Compatible:</b> Works seamlessly with any async runtime (<code>Tokio</code>, <code>async-std</code>, etc.) without special support or additional features - just time any expression, sync or async.</li>
    <li><b>Nanosecond Precision:</b> Uses platform-specific high-resolution timers through <code>std::time::Instant</code>, providing nanosecond-precision measurements with monotonic guarantees.</li>
    <li><b>Simple Statistics:</b> Provides essential statistics (<code>count</code>, <code>total</code>, <code>min</code>, <code>max</code>, <code>mean</code>) without complex algorithms or memory allocation, keeping the library focused and efficient.</li>
    <li><b>Production Metrics (optional):</b> Enable the <code>metrics</code> feature for a thread-safe <code>Watch</code>, <code>Timer</code> (auto record on drop), and <code>stopwatch!</code> macro powered by <code>hdrhistogram</code> for percentiles.</li>
    <li><b>Minimal API Surface:</b> Just four functions and two macros - easy to learn, hard to misuse, and unlikely to ever need breaking changes.</li>
    <li><b>Cross-Platform:</b> Consistent behavior across <b>Linux</b>, <b>macOS</b>, <b>Windows</b>, and other platforms supported by Rust's standard library.</li>
</ul>

<br>

<h2>Usage:</h2>

### Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
benchmark = "0.5.0"
```

#### Disable Default Features
```toml
[dependencies]
# Disable default features for true zero-overhead
benchmark = { version = "0.5.0", default-features = false }
```

<br>

## Quick Start

> <b>Feature flags</b>
> - <b>Default</b>: `features = ["std", "enabled"]`.
> - <b>enabled</b>: turns on timing (disable for true zero-overhead no-ops).
> - <b>std</b>: use Rust standard library (disables `no_std`).
> - <b>metrics</b>: production metrics (`Watch`, `Timer`, `stopwatch!`) powered by `hdrhistogram` (implies `std`).
> - <b>full</b>: convenience feature equal to `std + enabled`.
> - <b>minimal</b>: minimal core timing only; use with `default-features = false`.
> - For benches/examples in this README, run with: `--features "std enabled"`.

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

<br>

### Named timing + Collector aggregation (std + enabled)
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
The macros inline `Instant` timing when `features = ["std", "enabled"]`, so awaiting inside works seamlessly.
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

<hr>

<h2>Safety &amp; Edge Cases</h2>
<ul>
  <li><b>Zero durations</b>: Some operations can complete so fast that <code>elapsed()</code> may be 0ns on some platforms. The API preserves this and returns <code>Duration::ZERO</code>. If you need a floor for visualization, clamp in your presentation layer.</li>
  <li><b>Saturating conversions</b>: <code>Watch::record_instant()</code> converts <code>as_nanos()</code> to <code>u64</code> using saturating semantics, avoiding panics on extremely large values.</li>
  <li><b>Range clamping</b>: <code>Watch::record()</code> clamps input to histogram bounds, ensuring valid recording and protecting against out-of-range values.</li>
  <li><b>Drop safety</b>: <code>Timer</code> records exactly once. It guards against double-record by storing <code>Option&lt;Instant&gt;</code> and recording on <code>Drop</code> even during unwinding.</li>
  <li><b>Empty datasets</b>: <code>Watch::snapshot()</code> and <code>Collector</code> handle empty sets defensively. Snapshots for empty histograms return zeros; <code>Collector::stats()</code> returns <code>None</code> for missing keys.</li>
  <li><b>Overflow protection</b>: <code>Collector</code> uses <code>saturating_add</code> for total accumulation and 128-bit nanosecond storage in <code>Duration</code> to provide ample headroom.</li>
  <li><b>Thread safety</b>: All shared structures use <code>RwLock</code> with short hold-times: clone under read lock, compute outside the lock. Methods will panic only if a lock is poisoned by a prior panic.</li>
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
  <li><b>Size comparison (CI)</b>: Workflow <code>.github/workflows/ci.yml</code>, job <b>Zero Overhead</b> builds the crate twice and compares <code>libbenchmark.rlib</code> sizes with and without <code>enabled</code>. Any unexpected growth fails the job.</li>
  <li><b>Assembly artifacts (CI)</b>: Job <b>Assembly Inspection</b> emits <code>objdump</code>/<code>llvm-objdump</code> symbol tables and disassembly for enabled vs disabled. Download the <b>assembly-artifacts</b> artifact from the run to inspect.</li>
  <li><b>Runtime comparison (CI)</b>: Bench workflow <code>.github/workflows/bench.yml</code> runs the example <code>examples/overhead_compare.rs</code> in both modes and uploads <b>overhead-compare</b> artifacts with raw output.</li>
  <li><b>Compile tests (trybuild)</b>: <code>tests/trybuild_disabled.rs</code> ensures disabled mode compiles with the macro and function APIs used in a minimal program.</li>
  <li><b>Local reproduction</b>:
    <ul>
      <li>Disabled: <code>cargo run --release --example overhead_compare --no-default-features</code></li>
      <li>Enabled: <code>cargo run --release --example overhead_compare --no-default-features --features "std enabled"</code></li>
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
  <li>Use <code>--features std,enabled</code> to ensure the enabled timing path is benchmarked.</li>
</ul>

<h3>Benchmarks included</h3>
<ul>
  <li><b>overhead::instant</b>: <code>Instant::now().elapsed()</code> baseline.</li>
  <li><b>overhead::measure</b>: <code>measure(|| expr)</code>.</li>
  <li><b>overhead::time_macro</b>: <code>time!(expr)</code>.</li>
  <li>Bench source: <code>benches/overhead.rs</code>.</li>
  <li>Criterion version: <code>0.5</code>.</li>
  <li>Note: when features are disabled (<code>default-features = false</code>), measurement returns <code>Duration::ZERO</code>.</li>
  <li>Tip: use <code>--features std,enabled</code> to ensure the enabled path for benches.</li>
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
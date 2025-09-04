<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>Benchmark</b>
    <br>
    <sub>
        <sup>BENCHMARK SUITE</sup>
    </sub>
    <br>
</h1>
<div align="center">
    <sup>
    <a href="../README.md" title="Project Home"><b>HOME</b></a>
    <span>&nbsp;│&nbsp;</span>
    <a href="./README.md" title="Project Documentation"><b>DOCS</b></a>
    <span>&nbsp;│&nbsp;</span>
    <a href="./API.md" title="API Reference"><b>API</b></a>
    <span>&nbsp;│&nbsp;</span>
    <a href="./features/README.md" title="Feature Flags"><b>FEATURES</b></a>
    <span>&nbsp;│&nbsp;</span>
    <a href="./METRICS.md" title="Performance Metrics"><b>METRICS</b></a>
    </sup>
</div>

<br>

> Feature gating: This suite is enabled by the `benchmark` feature (default). For a minimal build, disable default features; re-enable with `-F benchmark` when needed.

<p>
    Benchmark is a development-time performance toolkit focused on simplicity and statistical rigor. Use it to time code paths, gather stable measurements, compare implementations, and catch regressions in CI. It favors minimal overhead and ergonomic APIs while remaining easy to disable entirely for zero-cost builds.
</p>

<br>

<hr>

<h2>Table of Contents</h2>
<ul>
    <li>
        <a href="#statistical-benchmarking"><b>Statistical Benchmarking</b></a>
    </li>
    <li>
        <a href="#benchmark-features"><b>Features</b></a>
        <ul>
            <li>
                <a href="#micro-benchmarking">Micro-Benchmarking</a>
            </li>
            <li>
                <a href="#macro-benchmarking">Macro-Benchmarking</a>
            </li>
            <li>
                <a href="#comparative-analysis">Comparative Analysis</a>
            </li>
            <li>
                <a href="#statistical-sampling">Statistical Sampling</a>
            </li>
            <li>
                <a href="#ci-cd-integration">CI/CD Integration</a>
            </li>
            <li>
                <a href="#load-testing">Load Testing</a>
            </li>
        </ul>
    </li>
</ul>

<hr>
<br>

<h2 id="measured-results">Measured Results (local)</h2>
<p>
  The following results were captured locally on Aug 29, 2025 using <code>cargo bench</code> with Criterion. Numbers are illustrative and may vary across machines and runs. See commands below to reproduce.
  <br>
</p>

<details>
<summary><b>Aggregation: benches/stats.rs</b></summary>

<pre>
stats::single/1000      time:   [1.9788 µs 2.0698 µs 2.2386 µs]
stats::single/10000     time:   [21.927 µs 22.630 µs 23.999 µs]

stats::all/k10_n1000    time:   [25.206 µs 25.520 µs 26.008 µs]
stats::all/k50_n1000    time:   [140.16 µs 145.45 µs 153.03 µs]

stats::array/k1_n10000  time:   [16.537 µs 17.437 µs 19.266 µs]
stats::array/k10_n1000  time:   [15.531 µs 16.423 µs 17.718 µs]
</pre>

<p>
  Notes:
  <br>
  • <em>array</em> variants avoid locking and are faster as expected.
  <br>
  • Multi-key <em>all</em> shows overhead from aggregation across keys.
  <br>
</p>
</details>

<details>
<summary><b>Allocations (placeholder)</b></summary>

<p>
  <em>TBD: Fill in allocation counts/bytes after running Instruments (Allocations) on benches:</em>
  <br>
  • <code>overhead</code> — core hot path (time!/measure)
  <br>
  • <code>stats</code> — aggregation paths
  <br><br>
  Suggested command:
  <br>
  <code>cargo instruments --bench overhead --template Allocations --time-limit 10</code>
  <br>
  <code>cargo instruments --bench stats --template Allocations --time-limit 10</code>
  <br><br>
  Record: allocations/iteration and total bytes.
  <br>
  Example table to complete:
  <br>
  <pre>
  bench              allocs/iter   bytes/iter
  overhead           [TBD]         [TBD]
  stats              [TBD]         [TBD]
  </pre>
  <br>
  Notes: aim for zero allocs on hot paths; any non-zero should be explained (e.g., formatting, map growth).
  <br>
  
</p>
</details>

<details>
<summary><b>Contention Profile (placeholder)</b></summary>

<p>
  <em>TBD: Fill in top hotspots after running Instruments (Time Profiler) for <code>collector_contention</code> across threads [1,2,4,8,16].</em>
  <br><br>
  Suggested command:
  <br>
  <code>cargo bench --bench collector_contention</code>
  <br>
  <code>cargo instruments --bench collector_contention --template "Time Profiler" --time-limit 15</code>
  <br><br>
  Capture per-scenario:
  <br>
  • <strong>single_key</strong> (worst-case)
  <br>
  • <strong>many_keys</strong> (reduced contention)
  <br><br>
  Example summary to complete:
  <pre>
  threads  scenario      top hotspots (function -> %time)
  1        single_key    [TBD]
  2        single_key    [TBD]
  4        single_key    [TBD]
  8        single_key    [TBD]
  16       single_key    [TBD]

  1        many_keys     [TBD]
  2        many_keys     [TBD]
  4        many_keys     [TBD]
  8        many_keys     [TBD]
  16       many_keys     [TBD]
  </pre>
  <br>
  Notes: identify lock hotspots (e.g., map lookups, RwLock/Mutex), quantify scaling deltas.
  <br>
</p>
</details>

<details>
<summary><b>Contention: benches/collector_contention.rs</b></summary>

<p>
  Includes two scenarios across thread counts [1, 2, 4, 8, 16]:
  <br>
  • <strong>single_key</strong>: worst-case contention (all threads record to one key)
  <br>
  • <strong>many_keys</strong>: thread-local key per thread to reduce contention
  <br>
  Run locally:
  <br>
  <code>cargo bench --bench collector_contention</code>
  <br>
  Results will be summarized here after profiling runs.
  <br>
</p>
</details>

<br>
<hr>
<br>

<h2 align="center" id="statistical-benchmarking">Statistical Benchmarking</h2>
<p>
    Robust benchmarking requires repeated sampling, summary statistics, and outlier awareness. Benchmark provides <code>Measurement</code>, <code>Collector</code>, and <code>Stats</code> so you can time code precisely and compute stable aggregates such as min/max/mean. Named measurements let you segment results per operation.
</p>

<br><br>
<hr>


<h2 id="benchmark-features">Features</h2>
<p>
    The development suite focuses on micro and macro benchmarks, comparative analysis, repeatable sampling, and simple CI integration. It uses <code>std::time::Instant</code> for high-resolution timing and keeps the API surface minimal.
</p>

<br>

<h3 id="micro-benchmarking">🧩 Micro-Benchmarking</h3>
<p>
    Measure small functions or critical inner loops. Run many iterations, collect durations, and analyze mean and spread. Keep inputs deterministic and avoid global state.
</p>

```rust
use benchmark::{Collector};

fn parse_u64(s: &str) -> u64 { s.parse().unwrap_or_default() }

fn main() {
    let mut c = Collector::new();
    for _ in 0..50_000 {
        let (_, d) = benchmark::time!(parse_u64("123456"));
        c.record_duration("parse", d);
    }
    let s = c.stats("parse").unwrap();
    println!("count={} mean={}ns", s.count, s.mean.as_nanos());
}
```

<br>

<h3 id="macro-benchmarking">🧩 Macro-Benchmarking</h3>
<p>
    Benchmark end-to-end scenarios like request handling or batch processing. Capture realistic latencies across multiple system components.
</p>

```rust
use benchmark::{Collector};

fn handle_request() { std::thread::sleep(std::time::Duration::from_millis(3)); }

fn main() {
    let mut c = Collector::new();
    for _ in 0..1_000 {
        let (_, d) = benchmark::time!(handle_request());
        c.record_duration("req", d);
    }
    let s = c.stats("req").unwrap();
    println!("count={} min={}ns max={}ns", s.count, s.min.as_nanos(), s.max.as_nanos());
}
```

<br>

<h3 id="comparative-analysis">📊 Comparative Analysis</h3>
<p>
    Compare two or more implementations by sampling each separately and contrasting their summary statistics. Keep workloads identical.
</p>

```rust
use benchmark::Collector;

fn a(buf: &[u8]) -> usize { buf.iter().filter(|b| **b % 2 == 0).count() }
fn b(buf: &[u8]) -> usize { buf.chunks(2).map(|c| c.len()).sum() }

fn main() {
    let data = vec![0u8; 4096];
    let mut ca = Collector::new();
    let mut cb = Collector::new();
    for _ in 0..10_000 { let (_, d) = benchmark::time!(a(&data)); ca.record_duration("a", d); }
    for _ in 0..10_000 { let (_, d) = benchmark::time!(b(&data)); cb.record_duration("b", d); }
    let sa = ca.stats("a").unwrap();
    let sb = cb.stats("b").unwrap();
    println!("A mean={}ns | B mean={}ns", sa.mean.as_nanos(), sb.mean.as_nanos());
}
```
<br>

<h3 id="statistical-sampling">📈 Statistical Sampling</h3>
<p>
    Sampling over large iteration counts reduces noise and reveals distribution shape. Use <code>Collector</code> to store measurements and compute statistics.
</p>

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

<h3 id="ci-cd-integration">⚙️ CI/CD Integration</h3>
<p>
    Run benchmark checks in CI and assert limits (e.g., mean under threshold). Track trends over time using <code>cargo bench</code>-style workflows or custom scripts.
</p>

```bash
# Example GitHub Actions step
cargo test --features "std benchmark"
```
<br>

<h3 id="load-testing">⚖️ Load Testing</h3>
<p>
    Generate continuous load to exercise systems and approximate production behavior. Focus on stability, backpressure, and tail latency.
</p>

```rust
use benchmark::Collector;

fn io() { std::thread::sleep(std::time::Duration::from_millis(1)); }

fn main() {
    let mut c = Collector::new();
    for _ in 0..5_000 { let (_, d) = benchmark::time!(io()); c.record_duration("io", d); }
    let s = c.stats("io").unwrap();
    println!("min={}ns max={}ns", s.min.as_nanos(), s.max.as_nanos());
}
```



<hr>
<br>

<h2 id="how-to-run-perf-benchmarks">How to run perf benchmarks (Criterion)</h2>
<p>
  Perf-sensitive Criterion benches are opt-in and gated by a feature flag plus an environment variable to avoid noisy CI results by default.
  These run in scheduled CI via <code>.github/workflows/perf.yml</code>, and can be invoked locally as follows:
  <br>
</p>

```bash
# Run perf-gated benches locally
PERF_TESTS=1 cargo bench -F perf-tests

# Optionally target a specific bench
PERF_TESTS=1 cargo bench -F perf-tests timers
PERF_TESTS=1 cargo bench -F perf-tests histogram_hot
PERF_TESTS=1 cargo bench -F "perf-tests metrics" watch_timer_hot
```

<p>
  Bench groups provided:
</p>

- <strong>timers</strong> — `benches/timers.rs`
  - `Instant::now()` throughput
  - `Duration` arithmetic (add, sub, mul, div)
- <strong>histogram_hot</strong> — `benches/histogram_hot.rs`
  - `Histogram::record` hot-path
  - `Histogram::percentiles` extraction
- <strong>watch_timer_hot</strong> — `benches/watch_timer_hot.rs`
  - `Watch::record` and `Watch::record_instant` hot paths
  - `Watch::snapshot` scaling by metric count and samples
  - `Timer` drop-throughput (records on drop)

<small>
Note: When the <code>perf-tests</code> feature is disabled, benches compile with a no-op <code>main()</code> to avoid linkage errors.
</small>

<!--
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
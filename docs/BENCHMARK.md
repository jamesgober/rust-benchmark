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
    <li>
        <a href="#tbd"><b>link</b></a>
    </li>
</ul>

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





<!--
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
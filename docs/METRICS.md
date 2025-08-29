<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>Benchmark</b>
    <br>
    <sub>
        <sup>PERFORMANCE METRICS</sup>
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
    <a href="./BENCHMARK.md" title="Benchmark Suite"><b>BENCHMARKING</b></a>
    </sup>
</div>

<br>

> Feature gating: Production metrics are enabled by the `metrics` feature (or by the convenience `standard` feature). They are not part of the minimal default unless `standard` is selected.

<p>
    Production performance metrics for latency and throughput critical paths. Use <code>Watch</code>, <code>Timer</code>, and <code>stopwatch!</code> to record nanosecond timings with negligible overhead and export percentile snapshots for dashboards and alerts.
</p>

<br>

<hr>

<h2>Table of Contents</h2>
<ul>
    <li>
        <a href="#performance-metrics-observability"><b>Performance Metrics &amp; Observability</b></a>
    </li>
    <li>
        <a href="#metrics-features"><b>Features</b></a>
        <ul>
            <li>
                <a href="#code-instrumentation">Code Instrumentation</a>
            </li>
            <li>
                <a href="#distributed-tracing">Distributed Tracing</a>
            </li>
            <li>
                <a href="#real-time-metrics">Real-time Metrics</a>
            </li>
            <li>
                <a href="#health-check-metrics">Health Check Metrics</a>
            </li>
            <li>
                <a href="#apm-integration">APM Integration</a>
            </li>
        </ul>
    </li>
    <li>
        <a href="#exporting"><b>Exporting</b></a>
    </li>
</ul>

<hr>
<br>

<h2 align="center" id="performance-metrics-observability">Performance Metrics &amp; Observability</h2>
<p>
    Observability requires low-overhead, high-fidelity measurements. The metrics feature uses an internal, zero-dependency histogram with lock-free recording to capture latencies precisely while minimizing contention. Snapshots expose p50/p90/p95/p99/p999, min/max, mean, and count.
</p>

<br><br>
<hr>

<h2 id="metrics-features">Features</h2>
<p>
    Focused on production use: code instrumentation, real-time snapshots, health checks, and easy export to your existing monitoring stack. No external histogram dependency.
</p>
<br>

<h3 id="code-instrumentation">🧭 Code Instrumentation</h3>
<p>
    Add timing to hot paths and endpoints using <code>stopwatch!</code> or <code>Timer</code>. Name metrics with a stable, low-cardinality scheme (e.g., <code>http.GET:/users/:id</code>).
</p>

```rust
use benchmark::{stopwatch, Watch};

fn main() {
    let watch = Watch::new();
    stopwatch!(watch, "http.GET:/users/:id", {
        std::thread::sleep(std::time::Duration::from_millis(2));
    });
    let s = &watch.snapshot()["http.GET:/users/:id"];
    println!("count={} p95={}ns", s.count, s.p95);
}
```

<br>

<h3 id="distributed-tracing">⚙️ Distributed Tracing</h3>
<p>
    While this crate does not implement tracing, you can correlate timings with trace/span IDs by embedding them in metric names or exporting snapshots alongside trace context captured in your application.
</p>

<br>
<h3 id="real-time-metrics">⚡ Real-time Metrics</h3>
<p>
    Periodically call <code>Watch::snapshot()</code> on a background interval to emit metrics to logs, Prometheus textfiles, OpenTelemetry exporters, or a custom sink.
</p>

```rust
use benchmark::Watch;

fn export(w: &Watch) {
    for (name, s) in w.snapshot() {
        println!(
            "name={} count={} min={} p50={} p90={} p99={} max={} mean={:.1}",
            name, s.count, s.min, s.p50, s.p90, s.p99, s.max, s.mean
        );
    }
}
```

<br>
<h3 id="health-check-metrics">❤️ Health Check Metrics</h3>
<p>
    Track periodic probes (DB ping, cache get, queue poll) to detect degradation early. Evaluate p99 and max against SLOs.
</p>

```rust
use benchmark::Watch;

fn db_ping() { /* ... */ }

fn main() {
    let w = Watch::new();
    for _ in 0..60 {
        let start = std::time::Instant::now();
        db_ping();
        w.record("health.db.ping", start.elapsed().as_nanos() as u64);
    }
    let s = &w.snapshot()["health.db.ping"];
    println!("p99={}ns max={}ns", s.p99, s.max);
}
```

<br>
<h3 id="apm-integration">📊 APM Integration</h3>
<p>
    Integrate by transforming snapshots into your APM’s metric format. The histogram values are already percentiles, so export as summary metrics or discrete gauges per percentile.
</p>

```rust
// Pseudocode for Prometheus text format
// io_latency_p50{name="io"}  1200
// io_latency_p90{name="io"}  2500
// io_latency_p99{name="io"}  4100
```

<h2 id="exporting">Exporting</h2>
<p>
    Tips for reliable exporting:
</p>

- **Stable names**: keep metric cardinality low.
- **Shard if hot**: use per-core suffixes and merge offline.
- **Reset between windows**: use <code>Watch::clear()</code> after export to bound latency windows.

<br>

### Prometheus (text exposition format)
```rust
use benchmark::Watch;

fn prometheus_export(w: &Watch) -> String {
    let mut out = String::new();
    for (name, s) in w.snapshot() {
        // Example: convert percentiles to summary-like gauges
        out.push_str(&format!("benchmark_latency_p50{{name=\"{}\"}} {}\n", name, s.p50));
        out.push_str(&format!("benchmark_latency_p90{{name=\"{}\"}} {}\n", name, s.p90));
        out.push_str(&format!("benchmark_latency_p99{{name=\"{}\"}} {}\n", name, s.p99));
        out.push_str(&format!("benchmark_latency_max{{name=\"{}\"}} {}\n", name, s.max));
        out.push_str(&format!("benchmark_latency_mean{{name=\"{}\"}} {:.1}\n", name, s.mean));
        out.push_str(&format!("benchmark_latency_count{{name=\"{}\"}} {}\n", name, s.count));
    }
    out
}
```

### OpenTelemetry (conceptual example)
```rust
// Requires adding dependencies:
// opentelemetry, opentelemetry-sdk, opentelemetry-metrics (versions per your stack)
use benchmark::Watch;

fn export_otlp(w: &Watch) {
    // Pseudocode: acquire a Meter from your OTel SDK setup
    // let meter = global::meter("benchmark");
    // let p50 = meter.u64_gauge("benchmark.latency.p50").init();
    // ... create instruments as needed

    for (name, s) in w.snapshot() {
        // p50.record(s.p50, &[KeyValue::new("name", name.clone())]);
        // p90.record(s.p90, &labels);
        // p99.record(s.p99, &labels);
        // count.add(s.count as u64, &labels);
    }
}
```
<br>





<br>

<!--
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
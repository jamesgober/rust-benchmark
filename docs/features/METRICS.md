<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br><b>Metrics</b><br>
    <sub><sup>
        METRICS FEATURE
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

> Feature gating: `metrics` is opt-in. Enable with `-F metrics` or via the convenience `-F standard` set. Not included in the minimal default without `standard`.

<p>
The <b>metrics</b> feature provides production-grade latency metrics with near-zero overhead. It offers lock-free recording, nanosecond precision, configurable bounds, and percentile snapshots (p50/p90/p95/p99/p999) using a built-in, zero-dependency histogram.
</p>

<br>

## Purpose
- Real-time performance monitoring for services, endpoints, and background jobs
- Application startup and cold/hot path timing
- Health/heartbeat latency timers and SLO/SLA tracking
- Export-ready snapshots for logs, metrics, and dashboards

<br>

🧩 **API**: 
[**`Watch`**](../API.md#watch),
[**`Timer`**](../API.md#timer),
[**`stopwatch!`**](../API.md#stopwatch)

⚙️ **Implementation**: Built-in zero-dependency histogram with lock-free recording and nanosecond precision.

<br>

## Installation


### Manual installation:
```toml
[dependencies]
benchmark = { version = "0.7.2", features = ["metrics"] }
```
> ⚙️ Add directly to your `Cargo.toml`.

<br>

### Installation via terminal:
```bash
cargo add benchmark -F metrics
```
> ⚙️ Using the `cargo add` command.

<br>
<hr>
<br>

## Examples

### Quick start: record timings and snapshot percentiles
```rust
use benchmark::Watch; // features = ["metrics"] (or "standard")

fn main() {
    let watch = Watch::new();
    for _ in 0..1000 {
        let start = std::time::Instant::now();
        // do work
        std::thread::sleep(std::time::Duration::from_micros(50));
        watch.record("io", start.elapsed().as_nanos() as u64);
    }

    let snap = watch.snapshot();
    let s = &snap["io"];
    println!("count={} p50={}ns p99={}ns max={}ns mean={:.1}", s.count, s.p50, s.p99, s.max, s.mean);
}
```

<br>
<hr>
<br>

## Accuracy and Performance Trade-offs

- **Bounds selection**: Choose `lowest` and `highest` close to your SLOs. Tighter ranges improve percentile precision and reduce memory.
- **Input clamping**: `Watch::record()` clamps to histogram bounds. Percentile queries clamp `q` to [0.0, 1.0]. This avoids panics and yields sane results for out-of-range inputs.
- **Precision vs memory**: The built-in histogram is fixed-size and optimized for lock-free recording; memory usage grows with the configured range. Tune bounds rather than sampling frequency.
- **Contention**: Recording is lock-free; map access for new metric names may take a write lock once. Keep metric names stable and low-cardinality. Consider sharding hot metrics and merging snapshots offline if necessary.
- **Snapshots**: Percentiles are computed from cloned histograms outside locks to minimize contention. Snapshot costs scale with the number of metrics.
- **Zero durations**: Some platforms can return 0ns for extremely fast ops; this is preserved. Clamp in presentation if needed, not at collection.
- **Tracing hooks (optional)**: Enabling the `trace` feature adds lightweight, gated logging for hot-path events to aid debugging overhead; it is zero-cost when disabled.

### Ergonomic scoped timing with `stopwatch!`
```rust
use benchmark::{stopwatch, Watch};

fn main() {
    let watch = Watch::new();
    stopwatch!(watch, "render", {
        std::thread::sleep(std::time::Duration::from_millis(2));
    });
    let s = &watch.snapshot()["render"];
    assert!(s.count >= 1);
}
```

### Guard timer that records on drop
```rust
use benchmark::{Timer, Watch};

fn main() {
    let w = Watch::new();
    {
        let _t = Timer::new(w.clone(), "tick");
        // work...
        std::thread::sleep(std::time::Duration::from_millis(1));
    } // recorded automatically
    assert_eq!(w.snapshot()["tick"].count, 1);
}
```

### Async example
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









<br>

<!--
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
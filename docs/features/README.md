<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>Benchmark</b>
    <br>
    <sub>
        <sup>FEATURE FLAGS</sup>
    </sub>
    <br>
</h1>

<p>
    This page documents the feature flags that control Benchmark's build-time behavior. Features allow you to choose between a minimal, zero-overhead core and progressively richer capabilities like the statistical benchmarking suite and production metrics.
    Each feature set is additive and designed to maintain performance, safety, and simplicity.
</p>

<br><br>

<div align="center">
    <h2></h2>
    <sup>
    <a href="../../README.md" title="Project Home"><b>HOME</b></a>
    <span>&nbsp;│&nbsp;</span>
    <a href="../README.md" title="Project Documentation"><b>DOCS</b></a>
    <span>&nbsp;│&nbsp;</span>
    <a href="../API.md" title="API Reference"><b>API</b></a>
    <span>&nbsp;│&nbsp;</span>
    <a href="../BENCHMARK.md" title="Benchmark Suite"><b>BENCHMARKING</b></a>
    <span>&nbsp;│&nbsp;</span>
    <a href="../METRICS.md" title="Performance Metrics"><b>METRICS</b></a>
    </sup>
</div>

<br>

## Features
- **`benchmark`** (default): Real timing, macros, and development benchmarking.
- **`collector`** (default): Collector, `Stats`, and built-in histogram backend.
- **`metrics`** (optional): Production metrics (`Watch`, `Timer`, `stopwatch!`).
- **`high-precision`** (optional): Selects the high-precision histogram backend.
- **`hdr`** (optional): External HDR histogram backend; requires `high-precision`.
- **`parking-lot-locks`** (optional): Faster synchronization for hot paths.

Notes:
- `metrics` implies `collector`.
- `hdr` implies `high-precision` and enables the optional `hdrhistogram` dependency.
- `std` is an internal implementation detail implied by higher-level features.

<hr>
<br>

## Feature Matrix

| Capability / API                                | `benchmark` | `collector` | `metrics` | `high-precision` | `hdr` |
|-------------------------------------------------|:-----------:|:-----------:|:---------:|:----------------:|:-----:|
| Core macros: `time!`, `time_named!`             |      ✓      |             |           |                  |       |
| Statistical: `benchmark_block!`, `benchmark!`   |      ✓      |             |           |                  |       |
| Types: `Duration`, `Measurement`, `Stats`       |      ✓      |      ✓      |           |                  |       |
| Collector + built-in histogram                  |             |      ✓      |           |         ✓        |       |
| Production metrics: `Watch`, `Timer`, `stopwatch!` |           |             |    ✓     |         ✓        |   ✓   |
| External HDR histogram backend                   |             |             |           |                  |   ✓   |

Notes:
- `high-precision` selects the high-precision histogram backend used by collectors/metrics.
- `hdr` switches the histogram backend to `hdrhistogram`.


<!-- DEFAULT FEATURE
############################################# -->
<h2 id="default-feature">Default Features</h2>
<p>
    The default build enables <code>benchmark</code> and <code>collector</code> for turn-key development: real timing, statistics, and a built-in histogram. Disable default features for a true zero-overhead core.
</p>

### Installation
#### Manual installation:
```toml
[dependencies]
benchmark = "0.7.0" # default features enabled (benchmark + collector)
```
> ⚙️ Add directly to your `Cargo.toml`.


### Installation via terminal:
```bash
cargo add benchmark
```
> ⚙️ Using the `cargo add` command.

<br>

Enables the [**`benchmark`**](./BENCHMARK.md) feature.


<hr>
<br>

<h2 id="metrics-feature">Metrics Feature</h2>
<p>
    Enable production metrics with <code>metrics</code>. This brings in <code>Watch</code>, <code>Timer</code>, and the <code>stopwatch!</code> macro, backed by the histogram backend.
</p>

### Installation
#### Manual installation:
```toml
[dependencies]
benchmark = { version = "0.7.0", features = ["metrics"]}
```

#### Terminal
```bash
cargo add benchmark -F metrics
```


<hr>
<br>

<!-- DISABLE DEFAULT FEATURE
############################################# -->
<h2 id="disable-default-feature">Disable Default Features</h2>
<p>
    Build the true zero-overhead core by disabling default features. Timing APIs compile to no-ops and add zero bytes/time.
    Re-enable selectively (e.g., <code>-F benchmark</code>, <code>-F metrics</code>) when needed.
</p>

### Installation
#### Manual installation:
```toml
[dependencies]
benchmark = { version = "0.7.0", default-features = false }
```
> ⚙️ Add directly to your `Cargo.toml`.


### Installation via terminal:
```bash
cargo add benchmark --no-default-features
```
> ⚙️ Using the `cargo add` command.

<br>

Disables default features (<code>benchmark</code> + <code>collector</code>). Combine with <code>-F benchmark</code> and/or <code>-F metrics</code> to opt back in.


<hr>
<br>


<!-- NONE FEATURE: Removed. Use --no-default-features instead. -->


<br>

<!--
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
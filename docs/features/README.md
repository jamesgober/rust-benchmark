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
- **`std`** (*default*): Uses Rust standard library; disables `no_std`
- [**`benchmark`**](./BENCHMARK.md) (*default*): Enables default benchmark tools.
- [**`metrics`**](./METRICS.md) (*optional*): Enables production metrics (`Watch`, `Timer`, `stopwatch!`).
- [**`default`**](#default-feature): Convenience feature equal to `std + benchmark`
- [**`standard`**](#standard-feature): Convenience feature equal to `std + benchmark + metrics`
- **`minimal`**: Minimal build with core timing only (*no default features*)
- **`all`**: Enables all features (*includes: `std + benchmark + metrics`*)

### Extras
- [**`disable-default`**](#disable-default-feature): Disables default features (*`std` + `benchmark`*)

<hr>
<br>

## Feature Matrix

| Capability / API                  | `std` | `benchmark` | `metrics` | `default` | `standard` | `all` |
|-----------------------------------|:-----:|:-----------:|:---------:|:---------:|:----------:|:-----:|
| Core macros: `time!`, `time_named!` |  ✓   |      ✓      |           |     ✓     |     ✓      |   ✓   |
| Statistical: `benchmark_block!`, `benchmark!` |  ✓ | ✓ |           | ✓ | ✓ | ✓ |
| Types: `Duration`, `Measurement`, `Stats` | ✓ | ✓ |           | ✓ | ✓ | ✓ |
| Production metrics: `Watch`, `Timer`, `stopwatch!` | ✓ |           |    ✓    |     –     |     ✓      |   ✓   |
| Collectors (`Collector`)          |  ✓   |      ✓      |           |     ✓     |     ✓      |   ✓   |
| No-std core (disabled path)       |  –    |      –      |    –      |     –     |     –      |   –   |

Notes:
- `default` = `std + benchmark`
- `standard` = `std + benchmark + metrics`
- `minimal` implies `--no-default-features` with no extras


<!-- DEFAULT FEATURE
############################################# -->
<h2 id="default-feature">Default Feature</h2>
<p>
    The default build enables <code>std</code> and <code>benchmark</code>. This provides core timing, macros, and the development benchmarking toolkit with zero configuration required.
    Disable default features for a true zero-overhead core.
    
</p>

### Installation
#### Manual installation:
```toml
[dependencies]
benchmark = "0.5.8" # default features enabled.

# Default features are enabled automatically; no extra configuration needed.
```
> ⚙️ Add directly to your `Cargo.toml`.


### Installation via terminal:
```bash
# default features enabled.
cargo add benchmark
```
> ⚙️ Using the `cargo add` command.

<br>

Enables the [**`benchmark`**](./BENCHMARK.md) feature.


<hr>
<br>

<!-- STANDARD FEATURE
############################################# -->
<h2 id="standard-feature">Standard Feature</h2>
<p>
    Convenience feature to enable the full development experience: <code>benchmark</code> (development) + <code>metrics</code> (production observability).
    The <code>metrics</code> feature implies <code>std</code> and uses an internal zero-dependency histogram.
</p>

### Installation
#### Manual installation:
```toml
[dependencies]
benchmark = { version = "0.5.8", features = ["standard"]}
```
> ⚙️ Add directly to your `Cargo.toml`.


### Installation via terminal:
```bash
cargo add benchmark -F standard
```
> ⚙️ Using the `cargo add` command.

<br>

Enables both the [**`benchmark`**](./BENCHMARK.md) and the [**`metrics`**](./METRICS.md) features.


<hr>
<br>

<!-- DISABLE DEFAULT FEATURE
############################################# -->
<h2 id="disable-default-feature">Disable Default Feature</h2>
<p>
    Build the true zero-overhead core by disabling default features. Timing APIs compile to no-ops and add zero bytes/time.
    Re-enable selectively (e.g., <code>-F benchmark</code>, <code>-F metrics</code>) when needed.
</p>

### Installation
#### Manual installation:
```toml
[dependencies]
benchmark = { version = "0.5.8", default-features = false }
```
> ⚙️ Add directly to your `Cargo.toml`.


### Installation via terminal:
```bash
# or set default directly. 
cargo add benchmark --no-default-features
```
> ⚙️ Using the `cargo add` command.

<br>

Disables <code>default</code> (which includes <code>benchmark</code>). Combine with <code>-F benchmark</code> or <code>-F metrics</code> to opt back in.


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
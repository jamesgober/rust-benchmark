<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>Benchmark</b>
    <br>
    <sub>
        <sup>API REFERENCE</sup>
    </sub>
    <br>
</h1>

<br>

## Table of Contents
- [Installation](#installation)

<br><br>

## Installation

### Install Manually

Add this to your `Cargo.toml`:
```toml
[dependencies]
benchmark = "0.1.5"
```

#### Disable Default Features
```toml
[dependencies]
# Disable default features for true zero-overhead
benchmark = { version = "0.1.5", default-features = false }
```

<br>

### Install via Terminal
```bash
# Basic installation (benchmarking feature only)
cargo add benchmark
```

#### Terminal: Disable Default Features
```bash
# Explicitly disabled - zero overhead
cargo add benchmark --no-default-features
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
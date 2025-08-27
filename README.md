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
</div>

<p>
    A zero-dependency, high-performance time measurement library for Rust that provides nanosecond precision benchmarking with true zero-overhead when disabled. Designed as a foundational primitive that other libraries can depend on without concern for bloat, version conflicts, or performance impact. This library follows the Unix philosophy of doing one thing well: measuring execution time with minimal overhead and maximum simplicity, making it suitable for embedding in performance-critical base libraries, async applications, and production systems.
</p>

<br>

<h2>Features</h2>
<ul>
    <li><b>True Zero-Overhead:</b> When disabled via <code>default-features = false</code>, all benchmarking code compiles away completely, adding zero bytes to your binary and zero nanoseconds to execution time.</li>
    <li><b>No Dependencies:</b> Built using only the Rust standard library, eliminating dependency conflicts and keeping compilation times fast.</li>
    <li><b>Thread-Safe by Design:</b> Core measurement functions are pure and inherently thread-safe, with an optional thread-safe Collector for aggregating measurements across threads.</li>
    <li><b>Async Compatible:</b> Works seamlessly with any async runtime (Tokio, async-std, etc.) without special support or additional features - just time any expression, sync or async.</li>
    <li><b>Nanosecond Precision:</b> Uses platform-specific high-resolution timers through std::time::Instant, providing nanosecond-precision measurements with monotonic guarantees.</li>
    <li><b>Simple Statistics:</b> Provides essential statistics (count, total, min, max, mean) without complex algorithms or memory allocation, keeping the library focused and efficient.</li>
    <li><b>Minimal API Surface:</b> Just four functions and two macros - easy to learn, hard to misuse, and unlikely to ever need breaking changes.</li>
    <li><b>Cross-Platform:</b> Consistent behavior across Linux, macOS, Windows, and other platforms supported by Rust's standard library.</li>
</ul>

<br>

<h2>Usage:</h2>

### Installation
Add this to your `Cargo.toml`:

```toml
[dependencies]
benchmark = "0.2.0"
```

#### Disable Default Features
```toml
[dependencies]
# Disable default features for true zero-overhead
benchmark = { version = "0.2.0", default-features = false }
```


<br>

## Quick Start

### {Example_Title}
{Example_Text}
```rust
{Example_Code}
```

<br>

### {Example_Title}
{Example_Text}
```rust
{Example_Code}
```

<!-- API REFERENCE
############################################# -->
<hr>

<h3>Documentation:</h3>
<ul>
    <li><a href="./docs/API.md"><b>API Reference</b></a> Complete documentation and examples.</li>
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
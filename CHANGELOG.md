<h1 align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <b>CHANGELOG</b>
</h1>
<p>
  All notable changes to this project will be documented in this file. The format is based on <a href="https://keepachangelog.com/en/1.1.0/">Keep a Changelog</a>,
  and this project adheres to <a href="https://semver.org/spec/v2.0.0.html/">Semantic Versioning</a>.
</p>

## [Unreleased]

### Fixed
- Disabled-path `benchmark!` macro parse error under `collector`-only builds (no `benchmark`): corrected macro expansion to use `$($body)*` in disabled arms.
- Clippy warnings in `src/trace.rs` (`inline_always`, `uninlined_format_args`): replaced with `#[inline]` and inline format args.
- Perf CI: baseline comparison script now supports both Criterion layouts (`target/criterion/<bench_name>/...` and flat `target/criterion/...`); fixes "criterion group directory not found" error in `.github/workflows/perf.yml`.

### Maintenance
- Bench workflow `.github/workflows/bench.yml`: use `examples/zero_overhead.rs` for the no-default-features run, and keep `overhead_compare` for the enabled run to avoid feature gating conflicts.
- Module order clean-up in `src/lib.rs` to satisfy Code Quality check (place `mod trace;` after `mod timer;`).
- Documentation consistency sweep: ensure install snippets reference `0.7.1` across `docs/` feature pages and API.


<br>

## [0.7.1] - 2025-08-30
### Added
- Optional `trace` feature providing lightweight internal trace hooks for debugging overhead.
  - Crate-private `trace::record_event()`; zero cost when feature is off.
  - Wired into `Watch::record()` fast/slow paths behind `cfg(feature = "trace")`.

### Fixed
- Eliminated `dead_code` warnings for `HistBackend` by gating the module behind `collector + metrics` in `src/lib.rs`.
- Removed an incorrect public `trace!` macro to avoid unresolved symbol/API surface growth.

### Maintenance
- Macro forward-compatibility: accepted optional trailing commas across timing/benchmark macros.


<br>

## [0.7.0] - 2025-08-30
### Changed
- BREAKING: Refactored feature flags to a clearer model. New flags:
  - `benchmark` (default): enables real timing and benchmarking macros; implies `std` internally.
  - `collector` (default): enables `Collector`, `Stats`, and built-in histogram; implies `std` internally.
  - `metrics`: enables production metrics (`Watch`, `Timer`, `stopwatch!`); implies `collector`.
  - `high-precision`: enables high-precision histogram backend; implies `collector`.
  - `hdr`: enables external HDR histogram backend (optional dep `hdrhistogram`); requires `high-precision`.
  - Note: `std` is now an internal implementation detail implied by higher-level features.
- Removed legacy/alias features: `minimal`, `standard`, `all`, and public `std` flag.

### Added
- Optional dependency `hdrhistogram = "7"` behind the `hdr` feature.
- CI updated to test new feature matrix, including combinations like `metrics high-precision` and `metrics hdr`.

### Migration
- For benchmarking-only users: no change needed (defaults remain timing + in-process stats).
- For production metrics: replace `features = ["standard"]` with `features = ["metrics"]`.
- For consumers of `Collector`/histogram only: use `features = ["collector"]`.



<br>

## [0.6.0] - 2025-08-29
### Added
- Scheduled perf workflow: `.github/workflows/perf.yml`
  - Runs twice weekly (Sun/Wed 02:00 UTC) and via manual dispatch
  - Gated with `PERF_TESTS=1` and `-F perf-tests`; runs perf-gated tests and benches
- Criterion benches (perf-gated):
  - `benches/timers.rs` — `Instant::now` throughput and `Duration` ops micro-benches
  - `benches/histogram_hot.rs` — `Histogram::record` and `Histogram::percentiles`
- Platform documentation: `docs/platforms/INSTANT.md` (platform-specific `Instant` notes)
- Cross-platform timer tests: `tests/platform_time.rs` (monotonicity, resolution semantics)
- Docs navigation: added “PLATFORMS” link in `docs/README.md`
- Manual observation test (opt-in): `tests/system_time_change.rs` (ignored; gated by `PERF_TESTS=1`)
- Criterion benches (metrics hot paths): `benches/watch_timer_hot.rs` (group `watch_timer_hot`)

### Changed
- CI perf workflow restricted to `main` and protected with concurrency guard
- Replaced `#[inline(always)]` with `#[inline]` for hot-path methods in `src/histogram.rs`
  - Resolves `clippy::inline_always` under `-D warnings` while preserving optimizer freedom
- `docs/API.md`: moved the Histogram section under “Types → Stats → Histogram” and added a runnable example with feature-gating notes
- `docs/features/README.md`: clarified `--no-default-features` usage comment and guidance to opt back in with `-F benchmark` / `-F metrics`
- `docs/BENCHMARK.md` and `docs/METRICS.md`: added consistent feature-gating notes near the top
- `docs/features/BENCHMARK.md` and `docs/features/METRICS.md`: added consistent feature-gating notes; fixed header label in Metrics page

### Documentation
- Doc sweep for clarity and consistency on feature gates across Benchmark and Metrics docs
- Emphasized opt-in perf gates: `perf-tests` with `PERF_TESTS=1`
- Clarified and emphasized that histogram percentile inputs are clamped to [0.0, 1.0] across `README.md` and `docs/API.md`. Out-of-range inputs map to min/max.

### Fixed
- Clippy lints in benches:
  - `benches/timers.rs`: use assign-op `+=` pattern
  - `benches/histogram_hot.rs`: removed unnecessary casts
- Added explicit ignore reasons for perf-gated tests/benches in `src/histogram.rs`
- Percentile interpolation overflow in `src/histogram.rs` when handling large bucket widths/counts: switched to u128 intermediates to prevent overflow.
- Ensured percentile 1.0 returns the true maximum in `Histogram::percentiles()` (explicit post-pass to set p==1.0 to max), aligning with `percentile(1.0)` behavior.

### Maintenance
- CI locally: `cargo fmt --all -- --check`, `cargo clippy --all-targets --all-features`, `cargo test --all-features`, `cargo doc --no-deps`




<br>

## [0.5.8] - 2025-08-28

### Added
- Macros: `benchmark_block!` and `benchmark!` for statistical benchmarking
  - `benchmark_block!` returns `Vec<Duration>` per-iteration timings (default 10_000 iterations)
  - `benchmark!` returns `(Option<T>, Vec<Measurement>)` with a name label
  - Both support async bodies and have zero-overhead disabled paths
- Tests: comprehensive coverage for enabled/disabled modes and iteration counts in `tests/macro_tests.rs`
- Documentation: usage examples and API entries in `README.md`, `docs/API.md`, and `docs/features/BENCHMARK.md`

### Maintenance
- Ran clippy and rustfmt; confirmed no new lints for the new macros


<br>

## [0.5.7] - 2025-08-28


### Added
- Changelog entry documenting histogram migration and zero-dependency metrics.

### Changed
- Metrics feature now uses built-in, zero-dependency histogram (`src/histogram.rs`) instead of `hdrhistogram`.
- `src/watch.rs`: refactored to store `Arc<Histogram>` per metric, lock-free recording on hot path, and updated `snapshot()` to internal API.
- `Cargo.toml`: removed `hdrhistogram` dependency and references from features; updated description; `all` now includes `metrics`.
- Documentation updated to reflect internal histogram: `README.md`, `docs/API.md`, `docs/features/METRICS.md`.

### Removed
- External dependency on `hdrhistogram` from the `metrics` feature (metrics is now zero-dependency).


<br>

## [0.5.6] - 2025-08-27

### Changed
- Updated project description to reflect dev, testing, and production use cases.
- Refined crate description in `Cargo.toml` to emphasize zero-overhead core and optional metrics (Watch/Timer, hdrhistogram).
- README intro expanded to highlight production observability path.

### Added
- README "Safety & Edge Cases" section documenting:
  - Zero-duration behavior, saturating conversions, histogram range clamping.
  - Drop-safety of `Timer`, defensive handling for empty datasets, overflow protection.
  - Thread-safety model and feature-gating guidance.

### Maintenance
- Verified build stability: `cargo fmt --all -- --check`, `cargo clippy`, and full `cargo test` matrix locally.

### Added
- Optional feature `metrics` providing production-friendly timing and metrics:
  - `watch.rs`: thread-safe `Watch` backed by `hdrhistogram` for percentile stats
  - `timer.rs`: `Timer` that auto-records elapsed time on `Drop`
  - `stopwatch!` macro for ergonomic timing (sync and async blocks)
- Documentation updates for metrics API (`docs/API.md`, `README.md`).
- Unit and async tests covering `Watch`, `Timer`, and `stopwatch!` macro.
- New builder API: `Watch::builder()` with `WatchBuilder` (implements `Default`) and `#[must_use]` on fluent setters.

### Fixed
- Corrected `hdrhistogram` dependency declaration: removed non-existent `std` feature.


<br>

## [0.5.0] - 2025-08-27

### Added
- Trybuild compile tests for disabled mode to assert API compiles in zero-overhead configuration:
  - `tests/trybuild_disabled.rs`
  - Fixture: `tests/trybuild/disabled_ok.rs`
- CI assembly inspection job to emit symbol tables and disassembly for enabled vs disabled builds:
  - Workflow: `.github/workflows/ci.yml` job "Assembly Inspection"
  - Artifacts: `assembly-artifacts` (objdump/llvm-objdump outputs)
- Dual-mode runtime comparison in bench workflow:
  - `.github/workflows/bench.yml` runs `examples/overhead_compare.rs` in disabled and enabled modes and uploads `overhead-compare` artifacts
- Example for overhead comparison:
  - `examples/overhead_compare.rs`
- README: “Zero-overhead Proof” section linking CI size comparison, assembly artifacts, and example outputs
- Criterion benchmarks for overhead analysis:
  - Bench file: `benches/overhead.rs`
  - Compares `Instant::now()` vs `measure` vs `time!`
- Cargo configuration updates:
  - `[dev-dependencies] criterion = "0.5"`
  - `[[bench]] name = "overhead"`, `harness = false`
- Criterion benchmarks for statistics aggregation:
  - Bench file: `benches/stats.rs`
  - Benchmarks `Collector::stats()` and `Collector::all_stats()` across varying sizes
- Array baseline aggregation benchmark (no locks) to compare overhead:
  - Added `stats::array` group in `benches/stats.rs`
- Async tests for macros and collector:
  - `tests/macro_tests.rs` adds async coverage for `time!` and `time_named!`
  - `tests/collector_async.rs` verifies collector with Tokio tasks
- CI: Scheduled (weekly) and manual benchmarks workflow:
  - `.github/workflows/bench.yml` runs `cargo bench` on nightly with `RUSTFLAGS=-C target-cpu=native`

### Changed
- `time!` and `time_named!` macros now inline timing under `features = ["std", "enabled"]` to support `await` inside macro bodies (async-friendly), preserving disabled zero-cost variants.
 - Performance: optimized `Collector::stats()` and `Collector::all_stats()`
   - Single-pass computation of total/min/max to reduce iterations
   - Clone under read lock, compute outside lock to reduce lock hold time and improve concurrency
   - Avoid nested locking in `all_stats()` by snapshotting data first


<br>

## [0.2.0] - 2025-08-27

### Added
- `collector.rs` file.
- `duration.rs` file.
- `measurement.rs` file.
- `integration_tests.rs` file in `tests`.
- Public API surface introduced:
  - Types: `Duration`, `Measurement`.
  - Functions: `measure`, `measure_named`.
  - Macros: `time!`, `time_named!`.
  - std-only types: `Collector`, `Stats` (behind `std` feature).
- Feature flags: `enabled`, `std`, `minimal`, `full` with `default = ["std", "enabled"]`.

### Changed
- BREAKING: `Collector::record` now accepts `&Measurement` instead of taking it by value: `pub fn record(&self, measurement: &Measurement)`.
  - Rationale: avoids unnecessary cloning/moves and enables cheaper call sites.
  - Migration: update call sites from `collector.record(measurement)` to `collector.record(&measurement)`.
- `Duration` Display implementation updated to use inline format args (no functional change).
- Documentation updated to reference version `0.2.0` and clarify feature usage (zero-overhead with `default-features = false`).
- Tests and examples updated to match the new `Collector::record(&Measurement)` signature.

### Fixed
- Resolved Clippy lints by adding targeted `#[allow(clippy::cast_precision_loss)]` and modernizing format strings; `#![deny(clippy::all)]` remains clean across all targets/features.
- Ensured `no_std` test stability by gating `test_duration_display` behind the `std` feature.


<br>

## [0.1.5] - 2025-08-26

Updated pre-dev release for backup.

### Added
- `docs/API.md` file.
- `docs/PRINCIPLES.md` file.
- `docs/README.md` file.
- This `CHANGELOG.md` file.
- GitHub CI Workflow `.github/workflows/ci.yml` file.

### Changed
- `Cargo.toml` file.
- `LICENSE` file.
- `README` file - Created basic structure.

### Removed
- `VERSION` file.


<br>

## [0.1.0] - 2025-08-19

Initial pre-dev release for backup.

### Added
- `Cargo.toml` file.
- `LICENSE` file.
- `VERSION` file.
- `README` file.




[Unreleased]: https://github.com/jamesgober/rust-benchmark/compare/v0.7.1...HEAD
[0.7.1]: https://github.com/jamesgober/rust-benchmark/compare/v0.7.0...v0.7.1
[0.8.0]: https://github.com/jamesgober/rust-benchmark/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/jamesgober/rust-benchmark/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/jamesgober/rust-benchmark/compare/v0.5.8...v0.6.0
[0.5.8]: https://github.com/jamesgober/rust-benchmark/compare/v0.5.7...v0.5.8
[0.5.7]: https://github.com/jamesgober/rust-benchmark/compare/v0.5.0...v0.5.7
[0.5.6]: https://github.com/jamesgober/rust-benchmark/compare/v0.5.0...v0.5.6
[0.8.0]: https://github.com/jamesgober/rust-benchmark/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/jamesgober/rust-benchmark/compare/v0.5.0...v0.7.0
[0.5.0]: https://github.com/jamesgober/rust-benchmark/compare/v0.2.0...v0.5.0
[0.2.0]: https://github.com/jamesgober/rust-benchmark/compare/v0.1.5...v0.2.0
[0.1.5]: https://github.com/jamesgober/rust-benchmark/compare/v0.1.0...v0.1.5
[0.1.0]: https://github.com/jamesgober/rust-benchmark/releases/tag/v0.1.0
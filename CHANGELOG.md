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

### Added
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

[Unreleased]: https://github.com/jamesgober/rust-benchmark/compare/v0.2.0...HEAD
[0.5.0]: https://github.com/jamesgober/rust-benchmark/compare/v0.2.0...v0.5.0
[0.2.0]: https://github.com/jamesgober/rust-benchmark/compare/v0.1.5...v0.2.0
[0.1.5]: https://github.com/jamesgober/rust-benchmark/compare/v0.1.0...v0.1.5
[0.1.0]: https://github.com/jamesgober/rust-benchmark/releases/tag/v0.1.0
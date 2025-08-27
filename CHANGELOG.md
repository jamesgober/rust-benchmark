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

[Unreleased]: https://github.com/jamesgober/rust-benchmark/compare/v0.2.0...HEAD
[0.3.0]: https://github.com/jamesgober/rust-benchmark/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/jamesgober/rust-benchmark/compare/v0.1.5...v0.2.0
[0.1.5]: https://github.com/jamesgober/rust-benchmark/compare/v0.1.0...v0.1.5
[0.1.0]: https://github.com/jamesgober/rust-benchmark/releases/tag/v0.1.0
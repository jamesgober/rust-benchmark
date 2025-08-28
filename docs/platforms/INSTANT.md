# std::time::Instant — Platform Notes

This document summarizes how `std::time::Instant` behaves across major platforms, with guidance for high-performance timing.

- macOS (Darwin)
  - Backed by `mach_absolute_time()` / `clock_gettime(CLOCK_MONOTONIC[_RAW])` depending on target.
  - Monotonic: yes. Immune to system time changes and NTP adjustments.
  - Resolution: typically sub-microsecond on modern Macs, but query-dependent. JIT and CPU frequency scaling can affect observed deltas.
  - Best practices: avoid converting to wall-clock; keep nanos as integers. Prefer batching `Instant::now()` calls in hot paths.

- Linux (glibc/musl)
  - Backed by `clock_gettime(CLOCK_MONOTONIC[_RAW])`.
  - Monotonic: yes. Does not jump with system time changes.
  - Resolution: commonly tens of nanoseconds to microseconds depending on kernel and hardware.
  - Best practices: pin CPU/governor for stable perf tests; avoid noisy shared runners for benchmarking.

- Windows
  - Backed by `QueryPerformanceCounter` (QPC).
  - Monotonic: yes (modern systems). Historically, some chipsets had issues pre-Vista; current systems are stable.
  - Resolution: high (sub-microsecond typical). Frequency exposed via QPC.
  - Best practices: avoid mixing with `GetTickCount`; rely on `Instant` for durations. Consider disabling turbo/boost for microbenchmarks.

General guidance
- `Instant` is monotonic and intended for durations; `SystemTime` can go backwards/forwards due to adjustments.
- Do not assume a fixed resolution; measure if needed. Observed minimum deltas are environment-dependent.
- For hot-paths, minimize `Instant::now()` calls; measure once, reuse, or pass durations explicitly.
- For perf tests, run on dedicated hardware, repeat with warmups, and compare distributions not single samples.

See also
- Perf benches for timer throughput and duration ops:
  - `benches/timers.rs` (group: `timers`)
  - How-to run: [docs/BENCHMARK.md#how-to-run-perf-benchmarks](../BENCHMARK.md#how-to-run-perf-benchmarks)

References
- Rust docs: https://doc.rust-lang.org/std/time/struct.Instant.html
- Linux `clock_gettime`: `man clock_gettime`
- Windows QPC: https://learn.microsoft.com/windows/win32/sysinfo/acquiring-high-resolution-time-stamps

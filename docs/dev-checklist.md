# Developer Profiling Checklist

This checklist captures the exact steps and data to collect for performance profiling. Keep results brief and paste summaries back into `docs/BENCHMARK.md` (Allocations and Contention sections).

## 1) Prereqs
- xcode-select --install
- brew install cargo-instruments

## 2) Allocations (memory usage)
Benchmarks: `overhead`, `stats`

Commands:
```bash
cargo instruments --bench overhead --template Allocations --time-limit 10
cargo instruments --bench stats    --template Allocations --time-limit 10
```
Capture:
- allocs/iteration
- bytes/iteration

Paste into `docs/BENCHMARK.md` → “Allocations (placeholder)” table:
```
bench     allocs/iter   bytes/iter
overhead  <NUM>         <BYTES>
stats     <NUM>         <BYTES>
```
Notes: Hot paths should be ~0 allocs; explain any non-zero (e.g., formatting, map growth).

## 3) Contention (time profiler)
Benchmark: `collector_contention` (scenarios: `single_key`, `many_keys`)

Run bench & profile:
```bash
cargo bench --bench collector_contention
cargo instruments --bench collector_contention --template "Time Profiler" --time-limit 15
```
Capture:
- For threads = [1,2,4,8,16]
- Top hotspots (function -> % time)
- Scaling differences between `single_key` and `many_keys`

Paste into `docs/BENCHMARK.md` → “Contention Profile (placeholder)” matrix:
```
threads  scenario      top hotspots (function -> %time)
1        single_key    ...
2        single_key    ...
4        single_key    ...
8        single_key    ...
16       single_key    ...

1        many_keys     ...
2        many_keys     ...
4        many_keys     ...
8        many_keys     ...
16       many_keys     ...
```

## 4) Finalize docs
- Update `docs/BENCHMARK.md` placeholders with collected numbers.
- Commit with message: "docs(bench): add allocations + contention results".

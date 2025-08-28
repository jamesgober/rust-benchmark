<div align="center">
    <img width="90px" height="auto" src="https://raw.githubusercontent.com/jamesgober/jamesgober/main/media/icons/hexagon-3.svg" alt="Triple Hexagon">
    <br>
    <h1>
        <strong>Benchmark</strong>
        <sup>
            <br>
            <sub>DOCUMENTATION</sub>
            <br>
        </sup>
    </h1>
</div>
<div align="center">
    <sup>
        <a href="../README.md" title="Project Home"><b>HOME</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./API.md" title="API Reference"><b>API</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./features/README.md" title="Feature Flags"><b>FEATURES</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./BENCHMARK.md" title="Performance Benchmark"><b>BENCHMARK</b></a>
        <span>&nbsp;│&nbsp;</span>
        <a href="./METRICS.md" title="Performance Metrics"><b>METRICS</b></a>
    </sup>
</div>


## Performance Tests (opt-in)

Perf-sensitive unit tests and benches are skipped by default to avoid host variance. Enable explicitly with both a feature flag and an environment variable:

```bash
# run perf tests (opt-in)
PERF_TESTS=1 cargo test -F perf-tests -- --ignored

# run benches that depend on perf gating
PERF_TESTS=1 cargo bench -F perf-tests
```

Notes:
- The `perf-tests` feature gates perf-sensitive paths in tests/benches.
- The `PERF_TESTS=1` env var is additionally checked inside tests to avoid accidental CI runs.





<!--
:: COPYRIGHT
============================================================================ -->
<div align="center">
  <br>
  <h2></h2>
  <sup>COPYRIGHT <small>&copy;</small> 2025 <strong>JAMES GOBER.</strong></sup>
</div>
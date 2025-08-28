#![allow(clippy::uninlined_format_args)]

// Criterion benches for Watch/Timer hot paths.
// Gated behind both `perf-tests` and `metrics` to avoid building unless explicitly enabled.

#[cfg(all(feature = "perf-tests", feature = "metrics"))]
use benchmark::Timer;
#[cfg(all(feature = "perf-tests", feature = "metrics"))]
use benchmark::Watch;
#[cfg(all(feature = "perf-tests", feature = "metrics"))]
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
#[cfg(all(feature = "perf-tests", feature = "metrics"))]
use std::time::Instant;

#[cfg(all(feature = "perf-tests", feature = "metrics"))]
fn bench_watch_record_hot(c: &mut Criterion) {
    let mut group = c.benchmark_group("watch/record");
    for &n in &[1_000_u64, 10_000, 100_000] {
        group.bench_with_input(format!("n={}", n), &n, |b, &iters| {
            b.iter_batched(
                Watch::new,
                |w| {
                    for _ in 0..iters {
                        w.record("op", 123);
                    }
                    w
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

#[cfg(all(feature = "perf-tests", feature = "metrics"))]
fn bench_watch_record_instant(c: &mut Criterion) {
    let mut group = c.benchmark_group("watch/record_instant");
    for &n in &[1_000_u64, 10_000, 100_000] {
        group.bench_with_input(format!("n={}", n), &n, |b, &iters| {
            b.iter(|| {
                let w = Watch::new();
                let start = Instant::now();
                for _ in 0..iters {
                    let t = start; // reuse to avoid now() in loop
                    w.record_instant("op", t);
                }
            })
        });
    }
    group.finish();
}

#[cfg(all(feature = "perf-tests", feature = "metrics"))]
fn bench_watch_snapshot_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("watch/snapshot_scaling");
    // (metrics_count, samples_per_metric)
    for &(m, s) in &[(1usize, 10_000usize), (10, 10_000), (100, 5_000)] {
        group.bench_with_input(format!("m={m} s={s}"), &(m, s), |b, &(metrics, samples)| {
            b.iter_batched(
                || {
                    let w = Watch::new();
                    for i in 0..metrics {
                        let name = format!("op{i}");
                        for _ in 0..samples {
                            w.record(&name, 123);
                        }
                    }
                    w
                },
                |w| {
                    let snap = w.snapshot();
                    criterion::black_box(snap)
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

#[cfg(all(feature = "perf-tests", feature = "metrics"))]
fn bench_timer_drop_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("timer/drop_throughput");
    for &n in &[1_000_u64, 10_000, 100_000] {
        group.bench_with_input(format!("n={n}"), &n, |b, &iters| {
            b.iter_batched(
                Watch::new,
                |w| {
                    for _ in 0..iters {
                        let _t = Timer::new(w.clone(), "op");
                        // drop at end of scope to record
                    }
                    w
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

#[cfg(all(feature = "perf-tests", feature = "metrics"))]
criterion_group!(
    watch_timer_hot,
    bench_watch_record_hot,
    bench_watch_record_instant,
    bench_watch_snapshot_scaling,
    bench_timer_drop_throughput
);
#[cfg(all(feature = "perf-tests", feature = "metrics"))]
criterion_main!(watch_timer_hot);

// When the feature set is not enabled, provide a no-op main to avoid linkage errors.
#[cfg(not(all(feature = "perf-tests", feature = "metrics")))]
fn main() {}

#![allow(clippy::uninlined_format_args)]

// Criterion benches for histogram hot paths. Gated by `perf-tests` so they run
// only in scheduled/manual perf jobs.

#[cfg(feature = "perf-tests")]
use benchmark::histogram::Histogram;
#[cfg(feature = "perf-tests")]
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

#[cfg(feature = "perf-tests")]
fn bench_histogram_record(c: &mut Criterion) {
    let mut group = c.benchmark_group("histogram/record");
    for &n in &[1_000_u64, 10_000, 100_000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &iters| {
            b.iter_batched(
                Histogram::new,
                |h| {
                    for i in 0..iters {
                        h.record((i as u64 % 1000) + 1);
                    }
                    h
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

#[cfg(feature = "perf-tests")]
fn bench_histogram_percentiles(c: &mut Criterion) {
    let mut group = c.benchmark_group("histogram/percentiles");
    let ps = [0.5, 0.9, 0.95, 0.99];
    for &n in &[10_000_u64, 100_000, 1_000_000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &iters| {
            b.iter_batched(
                || {
                    let h = Histogram::new();
                    for i in 0..iters {
                        h.record((i as u64 % 200_000) + 1);
                    }
                    h
                },
                |h| {
                    let vals = h.percentiles(&ps);
                    criterion::black_box(vals)
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

#[cfg(feature = "perf-tests")]
criterion_group!(
    histogram_hot,
    bench_histogram_record,
    bench_histogram_percentiles
);
#[cfg(feature = "perf-tests")]
criterion_main!(histogram_hot);

#[cfg(not(feature = "perf-tests"))]
fn main() {}

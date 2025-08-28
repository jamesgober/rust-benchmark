#![allow(clippy::uninlined_format_args)]

// Criterion benches for timer-related operations. Gated by the `perf-tests` feature
// so they only run in scheduled/manual perf jobs.

#[cfg(feature = "perf-tests")]
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
#[cfg(feature = "perf-tests")]
use std::time::{Duration, Instant};

#[cfg(feature = "perf-tests")]
fn bench_instant_now(c: &mut Criterion) {
    let mut group = c.benchmark_group("timers/instant_now");
    for &iters in &[1_u64 << 12, 1 << 14, 1 << 16] {
        group.bench_with_input(BenchmarkId::from_parameter(iters), &iters, |b, &n| {
            b.iter_batched(
                || (),
                |_| {
                    let mut last = Instant::now();
                    let mut progressed = 0_u64;
                    for _ in 0..n {
                        let now = Instant::now();
                        if now > last {
                            progressed += 1;
                        }
                        last = now;
                    }
                    criterion::black_box(progressed)
                },
                BatchSize::SmallInput,
            )
        });
    }
    group.finish();
}

#[cfg(feature = "perf-tests")]
fn bench_duration_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("timers/duration_ops");
    group.bench_function("add_sub", |b| {
        b.iter(|| {
            let mut d = Duration::from_nanos(1);
            for _ in 0..10_000 {
                d = d + Duration::from_nanos(1);
                d = d.saturating_sub(Duration::from_nanos(1));
            }
            criterion::black_box(d)
        })
    });
    group.finish();
}

#[cfg(feature = "perf-tests")]
criterion_group!(timers, bench_instant_now, bench_duration_ops);
#[cfg(feature = "perf-tests")]
criterion_main!(timers);

#[cfg(not(feature = "perf-tests"))]
fn main() {}

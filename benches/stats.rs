use benchmark::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn build_collector(keys: usize, per_key: usize) -> Collector {
    let c = Collector::with_capacity(keys);
    for k in 0..keys {
        let name: &'static str = Box::leak(format!("key_{k}").into_boxed_str());
        for i in 0..per_key {
            // distribute some values
            let v = (i as u128 % 1000) + 1;
            c.record_duration(name, Duration::from_nanos(v));
        }
    }
    c
}

fn build_arrays(keys: usize, per_key: usize) -> Vec<(&'static str, Vec<Duration>)> {
    let mut out = Vec::with_capacity(keys);
    for k in 0..keys {
        let name: &'static str = Box::leak(format!("key_{k}").into_boxed_str());
        let mut v = Vec::with_capacity(per_key);
        for i in 0..per_key {
            let d: u128 = (i as u128 % 1000) + 1;
            v.push(Duration::from_nanos(d));
        }
        out.push((name, v));
    }
    out
}

fn bench_stats_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("stats::single");
    for &size in &[1_000usize, 10_000] {
        // Prepare collector with 1 key of size `size`
        let coll = build_collector(1, size);
        group.bench_with_input(BenchmarkId::from_parameter(size), &coll, |b, coll| {
            b.iter(|| {
                let s = coll.stats("key_0").expect("present");
                black_box((
                    s.count,
                    s.total.as_nanos(),
                    s.min.as_nanos(),
                    s.max.as_nanos(),
                    s.mean.as_nanos(),
                ))
            })
        });
    }
    group.finish();
}

fn bench_stats_all(c: &mut Criterion) {
    let mut group = c.benchmark_group("stats::all");
    for &(keys, per_key) in &[(10usize, 1_000usize), (50, 1_000)] {
        let coll = build_collector(keys, per_key);
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("k{keys}_n{per_key}")),
            &coll,
            |b, coll| {
                b.iter(|| {
                    let all = coll.all_stats();
                    black_box(all.len())
                })
            },
        );
    }
    group.finish();
}

fn bench_array_aggregate(c: &mut Criterion) {
    // Compare pure aggregation without locks
    let mut group = c.benchmark_group("stats::array");
    for &(keys, per_key) in &[(1usize, 10_000usize), (10, 1_000)] {
        let arrays = build_arrays(keys, per_key);
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("k{keys}_n{per_key}")),
            &arrays,
            |b, arrays| {
                b.iter(|| {
                    let mut total_len = 0usize;
                    for (_name, v) in arrays.iter() {
                        total_len += v.len();
                        let mut total: u128 = 0;
                        let mut min = u128::MAX;
                        let mut max = 0u128;
                        for d in v.iter() {
                            let n = d.as_nanos();
                            total += n;
                            if n < min {
                                min = n;
                            }
                            if n > max {
                                max = n;
                            }
                        }
                        let _mean = total / v.len() as u128;
                        black_box((total, min, max));
                    }
                    black_box(total_len)
                })
            },
        );
    }
    group.finish();
}

fn criterion_benches(c: &mut Criterion) {
    bench_stats_single(c);
    bench_stats_all(c);
    bench_array_aggregate(c);
}

criterion_group!(benches, criterion_benches);
criterion_main!(benches);

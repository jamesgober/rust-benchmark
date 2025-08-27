use criterion::{black_box, criterion_group, criterion_main, Criterion};

use benchmark::*;

fn bench_instant(c: &mut Criterion) {
    let mut g = c.benchmark_group("overhead::instant");
    g.bench_function("instant_now_elapsed", |b| {
        b.iter(|| {
            let start = std::time::Instant::now();
            black_box(start.elapsed().as_nanos())
        })
    });
    g.finish();
}

fn bench_measure(c: &mut Criterion) {
    let mut g = c.benchmark_group("overhead::measure");
    g.bench_function("measure_closure_add", |b| {
        b.iter(|| {
            let (out, d) = measure(|| black_box(1usize) + black_box(1usize));
            black_box(out);
            black_box(d.as_nanos());
        })
    });
    g.finish();
}

fn bench_time_macro(c: &mut Criterion) {
    let mut g = c.benchmark_group("overhead::time_macro");
    g.bench_function("time_macro_add", |b| {
        b.iter(|| {
            let (out, d) = time!(black_box(1usize) + black_box(1usize));
            black_box(out);
            black_box(d.as_nanos());
        })
    });
    g.finish();
}

pub fn criterion_benches(c: &mut Criterion) {
    bench_instant(c);
    bench_measure(c);
    bench_time_macro(c);
}

criterion_group!(benches, criterion_benches);
criterion_main!(benches);

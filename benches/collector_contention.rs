#![allow(clippy::uninlined_format_args)]

#[cfg(not(feature = "std"))]
fn main() {}

#[cfg(feature = "std")]
use benchmark::*;
#[cfg(feature = "std")]
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
#[cfg(feature = "std")]
use std::sync::Arc;
#[cfg(feature = "std")]
use std::thread;

#[cfg(feature = "std")]
fn bench_collector_contention(c: &mut Criterion) {
    let mut group = c.benchmark_group("collector::contention");

    // Vary threads
    for &threads in &[1usize, 2, 4, 8, 16] {
        // Single key: worst-case contention on the inner Vec and map entry
        group.bench_with_input(
            BenchmarkId::new("single_key", threads),
            &threads,
            |b, &t| {
                b.iter_batched(
                    || Arc::new(Collector::new()),
                    |collector| {
                        let mut handles = Vec::with_capacity(t);
                        for i in 0..t {
                            let c = Arc::clone(&collector);
                            handles.push(thread::spawn(move || {
                                // Each thread records N entries to the same key
                                for j in 0..10_000u64 {
                                    let v = u128::from((i as u64) ^ j) % 1_000 + 1;
                                    c.record_duration("hot", Duration::from_nanos(v));
                                }
                            }));
                        }
                        for h in handles {
                            h.join().unwrap();
                        }
                        // Return total count to avoid being optimized away
                        let s = collector.stats("hot").unwrap();
                        criterion::black_box(s.count)
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        // Many keys: spread contention across map entries
        group.bench_with_input(BenchmarkId::new("many_keys", threads), &threads, |b, &t| {
            b.iter_batched(
                || Arc::new(Collector::new()),
                |collector| {
                    let mut handles = Vec::with_capacity(t);
                    for i in 0..t {
                        let c = Arc::clone(&collector);
                        handles.push(thread::spawn(move || {
                            // Each thread records into its own key
                            let key: &'static str = Box::leak(format!("key_{i}").into_boxed_str());
                            for j in 0..10_000u64 {
                                let v = u128::from((i as u64).wrapping_mul(31) ^ j) % 1_000 + 1;
                                c.record_duration(key, Duration::from_nanos(v));
                            }
                        }));
                    }
                    for h in handles {
                        h.join().unwrap();
                    }
                    // Return size to avoid being optimized away
                    let all = collector.all_stats();
                    criterion::black_box(all.len())
                },
                BatchSize::SmallInput,
            )
        });
    }

    group.finish();
}

#[cfg(feature = "std")]
criterion_group!(benches, bench_collector_contention);
#[cfg(feature = "std")]
criterion_main!(benches);

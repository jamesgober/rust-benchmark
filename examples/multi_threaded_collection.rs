// Multi-threaded collection using Collector.
// Default features already include std + benchmark.

use benchmark::{measure_named, Collector};
use std::thread;

fn work(n: u64) -> u64 {
    (0..n).fold(0u64, |acc, v| acc.wrapping_add(v.rotate_left(1)))
}

fn main() {
    let collector = Collector::new();

    let mut handles = Vec::new();
    for tid in 0..4u32 {
        let c = collector.clone();
        handles.push(thread::spawn(move || {
            for i in 0..10_000u64 {
                let (out, m) = measure_named("mt_work", || work((i % 10_000) + 1));
                // prevent optimizer from eliminating work
                std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::SeqCst);
                let _ = out ^ tid as u64; // touch result
                c.record(&m);
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let stats = collector.stats("mt_work").expect("stats present");
    println!(
        "threads recorded: count={}, min={}ns, mean={}ns, max={}ns",
        stats.count,
        stats.min.as_nanos(),
        stats.mean.as_nanos(),
        stats.max.as_nanos()
    );
}

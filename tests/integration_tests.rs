#![cfg(feature = "std")]

use benchmark::*;

#[test]
#[cfg_attr(miri, ignore = "sleep-based timing is slow/flaky under Miri")]
fn test_basic_timing() {
    let (result, duration) = measure(|| {
        std::thread::sleep(std::time::Duration::from_millis(1));
        42
    });

    assert_eq!(result, 42);

    #[cfg(feature = "benchmark")]
    {
        // Should take at least 1ms
        assert!(duration.as_millis() >= 1);
    }

    #[cfg(not(feature = "benchmark"))]
    {
        // Should be zero when disabled
        assert_eq!(duration.as_nanos(), 0);
    }
}

#[test]
fn test_time_macro() {
    let (result, duration) = time!(2 + 2);
    assert_eq!(result, 4);

    #[cfg(feature = "benchmark")]
    {
        // Touch duration so it's used under benchmark without triggering lints
        let _ = duration.as_nanos();
    }

    #[cfg(not(feature = "benchmark"))]
    assert_eq!(duration.as_nanos(), 0);
}

#[test]
#[cfg_attr(
    miri,
    ignore = "thread scheduling/timing can be non-deterministic under Miri"
)]
fn test_collector_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let collector = Arc::new(Collector::new());
    let mut handles = vec![];

    for i in 0..10 {
        let c = collector.clone();
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let measurement = Measurement {
                    name: "thread_test",
                    duration: Duration::from_nanos((i * 10 + j) as u128),
                    timestamp: 0,
                };
                c.record(&measurement);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let stats = collector.stats("thread_test").unwrap();
    assert_eq!(stats.count, 100);
}

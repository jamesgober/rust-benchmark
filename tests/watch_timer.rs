#![cfg(all(feature = "std", feature = "metrics"))]

use std::thread;
use std::time::Duration as StdDuration;

use benchmark::stopwatch;
#[allow(unused_imports)]
use benchmark::{Timer, Watch};

#[test]
fn watch_record_and_snapshot() {
    let watch = Watch::new();

    watch.record("op.a", 1_000);
    watch.record("op.a", 2_000);
    watch.record("op.b", 10_000);

    let snap = watch.snapshot();
    let a = snap.get("op.a").expect("op.a present");
    let b = snap.get("op.b").expect("op.b present");

    assert_eq!(a.count, 2);
    assert!(a.min >= 1_000 && a.max >= 2_000);
    assert_eq!(b.count, 1);
    assert!(b.min >= 10_000 && b.max >= 10_000);

    // basic sanity on percentiles
    assert!(a.p50 >= a.min && a.p999 <= a.max);
}

#[test]
fn timer_records_on_drop() {
    let watch = Watch::new();
    {
        let _t = Timer::new(watch.clone(), "timer.drop");
        thread::sleep(StdDuration::from_millis(2));
    }

    let snap = watch.snapshot();
    let s = snap.get("timer.drop").expect("timer.drop present");
    assert_eq!(s.count, 1);
    // at least ~2ms in ns
    assert!(s.min >= 2_000_000);
}

#[test]
fn stopwatch_macro_sync() {
    let watch = Watch::new();
    stopwatch!(watch, "macro.sync", {
        thread::sleep(StdDuration::from_millis(1));
    });
    let s = watch.snapshot().get("macro.sync").cloned().unwrap();
    assert_eq!(s.count, 1);
    assert!(s.min >= 1_000_000);
}

#[cfg(feature = "std")]
#[tokio::test(flavor = "multi_thread")]
async fn stopwatch_macro_async() {
    use tokio::time::{sleep, Duration as TokioDuration};

    let watch = Watch::new();
    stopwatch!(watch, "macro.async", {
        sleep(TokioDuration::from_millis(1)).await;
    });
    let s = watch.snapshot().get("macro.async").cloned().unwrap();
    assert_eq!(s.count, 1);
    assert!(s.min >= 1_000_000);
}

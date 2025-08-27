#![cfg(feature = "std")]

use benchmark::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_collector_with_tokio_tasks() {
    let collector = std::sync::Arc::new(Collector::new());

    let tasks = 16usize;
    let per_task = 50usize;

    let mut handles = Vec::with_capacity(tasks);
    for _ in 0..tasks {
        let c = collector.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..per_task {
                // simulate some async work
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
                c.record_duration("tokio_async", Duration::from_nanos(1));
            }
        }));
    }

    for h in handles {
        h.await.unwrap();
    }

    let stats = collector.stats("tokio_async").expect("stats exist");
    assert_eq!(stats.count as usize, tasks * per_task);
    // total should equal count * 1 ns
    assert_eq!(stats.total.as_nanos(), (tasks * per_task) as u128);
}

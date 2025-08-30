// Async timing using the time!/time_named! macros inside async code.
// Requires the default features (std + benchmark). Tokio is a dev-dependency.

use benchmark::{time, time_named, Duration};
use tokio::time::{sleep, Duration as TokioDuration};

#[tokio::main(flavor = "multi_thread")] // uses dev-dependency `tokio`
async fn main() {
    let ((), d1): ((), Duration) = time!({
        sleep(TokioDuration::from_millis(50)).await;
    });
    println!("awaited sleep 50ms in {} ({} ns)", d1, d1.as_nanos());

    let (out, m) = time_named!("async_compute", async_compute().await);
    println!(
        "time_named async_compute -> {out}, name={}, took {} ({} ns)",
        m.name,
        m.duration,
        m.duration.as_nanos()
    );
}

async fn async_compute() -> u64 {
    // Simulate IO + CPU
    sleep(TokioDuration::from_millis(10)).await;
    (0..100_000u64).sum()
}

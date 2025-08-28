#![cfg(feature = "std")]

use benchmark::*;

#[test]
fn test_time_named_macro() {
    let (result, m) = time_named!("named_add", 2 + 3);
    assert_eq!(result, 5);
    assert_eq!(m.name, "named_add");

    #[cfg(feature = "benchmark")]
    {
        // Touch duration to avoid lints and ensure it's available
        let _ = m.duration.as_nanos();
    }

    #[cfg(not(feature = "benchmark"))]
    {
        assert_eq!(m.duration.as_nanos(), 0);
        assert_eq!(m.timestamp, 0);
    }
}

#[test]
fn test_benchmark_block_defaults_and_iters() {
    // Default iters (10_000) when benchmark feature is on, else empty vec.
    let samples_default = benchmark_block!({
        let mut x = 0u64;
        x = x.wrapping_add(1);
        let _ = x;
    });

    #[cfg(feature = "benchmark")]
    {
        assert_eq!(samples_default.len(), 10_000);
        // Touch a couple of samples to ensure types and values are usable
        let _ = samples_default[0].as_nanos();
    }

    #[cfg(not(feature = "benchmark"))]
    {
        assert!(samples_default.is_empty());
    }

    // Explicit iters
    let iters = 123usize;
    let samples_iters = benchmark_block!(iters, {
        let mut y = 1u64;
        y = y.wrapping_mul(2);
        let _ = y;
    });

    #[cfg(feature = "benchmark")]
    {
        assert_eq!(samples_iters.len(), iters);
    }

    #[cfg(not(feature = "benchmark"))]
    {
        assert!(samples_iters.is_empty());
    }
}

#[test]
fn test_benchmark_macro_sync_expr_and_block() {
    // Expr form with default iterations
    let (out_default, ms_default) = benchmark!("add", { 2 + 3 });

    #[cfg(feature = "benchmark")]
    {
        assert!(out_default.is_some());
        assert_eq!(out_default.unwrap(), 5);
        assert_eq!(ms_default.len(), 10_000);
        // Ensure Measurement fields are accessible
        let m = &ms_default[0];
        assert_eq!(m.name, "add");
        let _ = m.duration.as_nanos();
    }

    #[cfg(not(feature = "benchmark"))]
    {
        assert_eq!(out_default, Some(5));
        assert!(ms_default.is_empty());
    }

    // iters + expr form
    let (out_iters, ms_iters) = benchmark!("mul", 77usize, { 6 * 7 });

    #[cfg(feature = "benchmark")]
    {
        assert_eq!(out_iters, Some(42));
        assert_eq!(ms_iters.len(), 77);
    }

    #[cfg(not(feature = "benchmark"))]
    {
        assert_eq!(out_iters, Some(42));
        assert!(ms_iters.is_empty());
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_time_macro_async() {
    async fn async_add() -> u64 {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        40 + 2
    }

    let (result, d) = time!(async_add().await);
    assert_eq!(result, 42);

    #[cfg(feature = "benchmark")]
    {
        assert!(d.as_millis() >= 1);
    }

    #[cfg(not(feature = "benchmark"))]
    {
        assert_eq!(d.as_nanos(), 0);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_time_named_macro_async() {
    async fn async_mul(a: u64, b: u64) -> u64 {
        tokio::time::sleep(std::time::Duration::from_millis(1)).await;
        a * b
    }

    let (result, m) = time_named!("async_mul", async_mul(6, 7).await);
    assert_eq!(result, 42);
    assert_eq!(m.name, "async_mul");

    #[cfg(feature = "benchmark")]
    {
        assert!(m.duration.as_millis() >= 1);
    }

    #[cfg(not(feature = "benchmark"))]
    {
        assert_eq!(m.duration.as_nanos(), 0);
        assert_eq!(m.timestamp, 0);
    }
}

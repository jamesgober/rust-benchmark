#![cfg(feature = "std")]

use benchmark::*;

#[test]
fn test_time_named_macro() {
    let (result, m) = time_named!("named_add", 2 + 3);
    assert_eq!(result, 5);
    assert_eq!(m.name, "named_add");

    #[cfg(feature = "enabled")]
    {
        // Touch duration to avoid lints and ensure it's available
        let _ = m.duration.as_nanos();
    }

    #[cfg(not(feature = "enabled"))]
    {
        assert_eq!(m.duration.as_nanos(), 0);
        assert_eq!(m.timestamp, 0);
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

    #[cfg(feature = "enabled")]
    {
        assert!(d.as_millis() >= 1);
    }

    #[cfg(not(feature = "enabled"))]
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

    #[cfg(feature = "enabled")]
    {
        assert!(m.duration.as_millis() >= 1);
    }

    #[cfg(not(feature = "enabled"))]
    {
        assert_eq!(m.duration.as_nanos(), 0);
        assert_eq!(m.timestamp, 0);
    }
}

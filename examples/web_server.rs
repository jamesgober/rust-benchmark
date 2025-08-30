// Minimal single-connection HTTP example showing request timing with Watch/Timer.
// Run with metrics enabled:
//   cargo run --example web_server --features metrics

#[cfg(all(feature = "std", feature = "metrics"))]
use std::io::Write;

#[cfg(all(feature = "std", feature = "metrics"))]
fn main() -> std::io::Result<()> {
    use benchmark::{stopwatch, Watch};
    use std::net::TcpListener;
    use std::time::Duration as StdDuration;
    use std::time::Instant;

    let watch = Watch::new();
    let listener = TcpListener::bind(("127.0.0.1", 0))?;
    // Fail-safe: don't block forever waiting for a client
    listener.set_nonblocking(true)?;
    let addr = listener.local_addr()?;
    println!("listening on http://{addr} (will accept 1 connection; timeout 5s)");

    // Accept a single request with timeout to keep the example simple.
    let start = Instant::now();
    let timeout = StdDuration::from_secs(5);
    loop {
        match listener.accept() {
            Ok((mut stream, _)) => {
                // Time the handling scope
                stopwatch!(watch, "request", {
                    handle_connection(&mut stream).ok();
                });
                break;
            }
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                if start.elapsed() >= timeout {
                    eprintln!("No connection received within {timeout:?}; exiting.");
                    break;
                }
                std::thread::sleep(StdDuration::from_millis(25));
                continue;
            }
            Err(e) => {
                eprintln!("Accept error: {e}; exiting.");
                break;
            }
        }
    }

    // Print metrics snapshot
    let snap = watch.snapshot();
    if let Some(stats) = snap.get("request") {
        println!(
            "handled 1 request: count={}, min={}ns, p50={}ns, p99={}ns, max={}ns",
            stats.count, stats.min, stats.p50, stats.p99, stats.max
        );
    }

    Ok(())
}

#[cfg(all(feature = "std", feature = "metrics"))]
fn handle_connection(stream: &mut std::net::TcpStream) -> std::io::Result<()> {
    // Simulate some work
    let (_out, _d) = benchmark::time!({
        // light CPU burn
        let mut x = 0u64;
        for i in 0..100_000 {
            x = x.wrapping_add(i);
        }
        x
    });

    let body = b"Hello, benchmark!\n";
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(resp.as_bytes())?;
    stream.write_all(body)?;
    Ok(())
}

#[cfg(not(all(feature = "std", feature = "metrics")))]
fn main() {
    eprintln!("This example requires --features metrics (and std).\nTry: cargo run --example web_server --features metrics");
}

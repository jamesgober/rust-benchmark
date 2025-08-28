// Run with: cargo run --example prometheus_textfile --features "std metrics"
// This writes a Prometheus text exposition file to target/metrics.prom by default.

#[cfg(all(feature = "std", feature = "metrics"))]
use std::fs;
#[cfg(all(feature = "std", feature = "metrics"))]
use std::io::Write;
#[cfg(all(feature = "std", feature = "metrics"))]
use std::path::PathBuf;
#[cfg(all(feature = "std", feature = "metrics"))]
use std::time::Duration as StdDuration;

#[cfg(all(feature = "std", feature = "metrics"))]
use benchmark::{stopwatch, Watch};

#[cfg(all(feature = "std", feature = "metrics"))]
fn prometheus_export(w: &Watch) -> String {
    let mut out = String::new();
    for (name, s) in w.snapshot() {
        // Convert percentiles and aggregates into gauges
        out.push_str(&format!(
            "benchmark_latency_p50{{name=\"{}\"}} {}\n",
            name, s.p50
        ));
        out.push_str(&format!(
            "benchmark_latency_p90{{name=\"{}\"}} {}\n",
            name, s.p90
        ));
        out.push_str(&format!(
            "benchmark_latency_p99{{name=\"{}\"}} {}\n",
            name, s.p99
        ));
        out.push_str(&format!(
            "benchmark_latency_max{{name=\"{}\"}} {}\n",
            name, s.max
        ));
        out.push_str(&format!(
            "benchmark_latency_mean{{name=\"{}\"}} {:.1}\n",
            name, s.mean
        ));
        out.push_str(&format!(
            "benchmark_latency_count{{name=\"{}\"}} {}\n",
            name, s.count
        ));
    }
    out
}

#[cfg(all(feature = "std", feature = "metrics"))]
fn main() -> std::io::Result<()> {
    // Build a small set of metrics
    let watch = Watch::new();

    stopwatch!(watch, "io.read", {
        std::thread::sleep(StdDuration::from_millis(2));
    });
    stopwatch!(watch, "io.write", {
        std::thread::sleep(StdDuration::from_millis(3));
    });

    // Export to Prometheus text format
    let text = prometheus_export(&watch);

    // Determine output path
    let mut path = PathBuf::from("target/metrics.prom");
    if let Some(arg1) = std::env::args().nth(1) {
        path = PathBuf::from(arg1);
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = fs::File::create(&path)?;
    f.write_all(text.as_bytes())?;

    eprintln!("Wrote {} bytes to {}", text.len(), path.display());
    Ok(())
}

#[cfg(not(all(feature = "std", feature = "metrics")))]
fn main() {
    eprintln!(
        "This example requires features: std + metrics.\nRun: cargo run --example prometheus_textfile --features \"std metrics\""
    );
}

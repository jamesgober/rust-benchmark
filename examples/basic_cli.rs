use benchmark::{measure, time, Duration};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    // Time an inline expression with the macro
    let (result, d): (u64, Duration) = time!(fibonacci(30));
    println!(
        "time!: fibonacci(30) = {result}, took {d} ({:?} ns)",
        d.as_nanos()
    );

    // Time a closure with the function API
    let (sum, d2) = measure(|| (0..1_000_000u64).sum::<u64>());
    println!(
        "measure: sum 0..1_000_000 = {sum}, took {d2} ({:?} ns)",
        d2.as_nanos()
    );
}

use benchmark::*;

fn main() {
    // Compare macro overhead in a tight loop.
    // Note: This example runs under both enabled and disabled modes.
    let iters = 1_0000;

    // time! macro test
    let (sum, d) = time!({
        let mut s = 0u64;
        for i in 0..iters {
            // do trivial work
            s = s.wrapping_add(i as u64);
        }
        s
    });

    println!("time_macro_ns={} sum={}", d.as_nanos(), sum);

    // measure() function test
    let (sum2, d2) = measure(|| {
        let mut s = 0u64;
        for i in 0..iters {
            s = s.wrapping_add(i as u64);
        }
        s
    });

    println!("measure_fn_ns={} sum2={}", d2.as_nanos(), sum2);
}

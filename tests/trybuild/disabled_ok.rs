fn main() {
    // Ensure APIs exist and compile in disabled mode (no-default-features)
    let (v, d) = benchmark::time!(1usize + 1usize);
    let (_v2, m) = benchmark::time_named!("add", 1usize + 1usize);
    let _n = d.as_nanos();
    let _nm = m.duration.as_nanos();
    let _ = v;
}

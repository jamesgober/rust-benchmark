use trybuild::TestCases;

#[test]
pub fn disabled_mode_compiles() {
    // This test relies on the cargo invocation features.
    // In CI we run it under --no-default-features (and also with other combos).
    // The fixture uses only APIs that must exist in disabled mode.
    let t = TestCases::new();
    t.pass("tests/trybuild/disabled_ok.rs");
}

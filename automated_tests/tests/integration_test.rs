// Each file in the tests directory is a different crate. This is why we need
// import the automated_tests module. You don't need a #[cfg(test)] because the
// test directory is only compiled using cargo test. Only projects with a lib.rs
// can have integration tests.
use automated_tests;

#[test]
fn it_adds_two() {
    assert_eq!(4, automated_tests::add(2, 2));
}

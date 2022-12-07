fn main() {
    // Manually force a unrecoverable error
    // panic!("crash and burn");

    // Try to access an index out of bounds.
    // To see the backtrace, run:
    // RUST_BACKTRACE=1 cargo run
    let v = vec![1, 2, 3];
    v[99];
}

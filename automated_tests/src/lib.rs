pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// By default Rust runs tests in parallel. Rust doesn't show output of tests that
// pass. This indicates that this code will only be compiled when running cargo
// test and not something like cargo build. Rust allows private functions to be
// tested.
#[cfg(test)]
mod tests {
    // This allows anything in the outer module to be available in the test module.
    use super::*;

    // The test annotation indicates that the function is a test function.
    #[test]
    fn it_works() {
        let result = add(2, 2);
        // The assert equals macro asserts a certain result should be equal to
        // something. Order does not matter in rust.
        assert_eq!(result, 4);

        // assert is used when you want something to evaluate to true. You can
        // also use assert_eq and assert_ne to compare two values. You can use
        // later parameters similar to a formatted print.
        assert!(result == 4, "This is an error message {}", result);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    // Indicate that this test should panic with a specific message.
    #[should_panic(expected = "Make this test fail")]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    // Tests can be written to use Result<T, E> to return an err instead of
    // panicking.
    #[ignore]
    // You can also ignore a test to prevent it being ran by cargo test.
    fn result_test() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err("two plus two does not equal four".to_string())
        }
    }
}

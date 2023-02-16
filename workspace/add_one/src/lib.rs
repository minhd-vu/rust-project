pub fn add_one(x: i32) -> i32 {
    x + 1
}

// cargo test will run tests for all crates in the workspace.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}

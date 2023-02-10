//! These types of comments are usually found in the crate root file. This
//! documents the file rather than a code snippet.

/// Triple slashes mean that whatever is here is inside is a document comment
/// that will be rendered using Markdown.
///
/// Output:
/// ```
/// Hello World!
/// ```
///
/// Use `cargo doc --open` to see the documentation for your crates.
/// The sections crate authors generally use are:
/// - Examples
/// - Panics
/// - Errors
/// - Safety
///
/// Test can also be put in documentation comments and ran with `cargo test`.
/// ```
/// assert_eq!(1, 1);
/// ```
///
fn main() {
    println!("Hello, world!");
}

// Pub use statements are good for reorganizing how the public api is presented.
// It prevents some items from being too deeply nested and makes it more
// ergonomic for users.
//
// pub use self::

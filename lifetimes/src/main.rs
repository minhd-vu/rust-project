// The lifetime of r is the entire main function.
fn main() {
    let r;

    // The lifetime of x is only within these curly braces.
    {
        let x = 5;
        r = &x;
    }

    // This won't compile because r is trying to access a variable that has gone
    // out of scope. It will state the variable does not live long enough. If
    // rust allowed this to work, r would be referencing memory that would have
    // have deallocated. You cannot reference memory with a shorter lifetime.
    println!("r: {}", r);
}

// This is the same as the main function, but will compile. There is no longer a
// dangling reference.
fn fixed() {
    let x = 5;
    let r = &x;
    println!("r: {}", r);
}

// This won't compile because the return type doesn't know which lifetime we are
// using: x's lifetime or y's lifetime.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This fixes the longest function above, by specifying the lifetime could be of
// either x or y which both have the same lifetime. A generic lifetime parameter.
// This means the lifetime is at least as long as 'a.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Rust has lifetime elision rules which are rules that will automatically apply
// lifetime annotations to prevent writing extra code. Note that rust will never
// infer lifetimes when it is ambiguous.


// This is a special lifetime called the static lifetime and can live for the 
// entire duration of the program. All string literals have the static lifetime.
let s: &'static str = "I have a static lifetime.";


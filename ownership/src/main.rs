fn main() {
    // s is not valid here, it’s not yet declared
    {
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // String manages data allocated on the heap.
    let s = String::from("hello");

    // for string literals, the value is known at compile time which makes them fast and efficient.
    // Memory is automatically returned when the variable that owns it goes out of scope.
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid.

    let s1 = String::from("hello");
    let s2 = s1; // s1 is no longer valid.
    // This is similar to a shallow copy, but it is called a move, as in s1 was moved into s2.
    // This allows only s2 to free the memory when s1 and s2 go out of scope.

    // The Copy trait signifies that variables that use it do not move, but rather are trivially copied.
    // Most primitive data types implement Copy, tuples do as well if all types of the tuple implement Copy.

    // Passing a variable to a function will move or copy it; therefore it will no longer be valid in the
    // current scope after the function call.
    // Returning a value will transfer ownership from the called function the function calling it.

    // References allow passing of variables without transferring ownership, also known as borrowing.
    // References are similar to pointers, but references are guaranteed to point to valid data.
    // References are represented with the ampersand and are immutable by default. If you want to modify
    // a borrowed variable, use a mutable reference. You may only have one mutable reference to a value
    // in the same scope. This allows code with data races to not compile. You cannot have mutable and
    // immutable references in the same scope of the same value. A reference’s scope starts from where 
    // it is introduced and continues through the last time that reference is used. Rust compiler
    // guarantees that references will never be a dangling pointer.
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s // If this was &s, it would cause a dangling pointer.
}

fn slices() {
    // A slice is a reference to part of a string. Slices are references that point to a part of a
    // underlying string literal with a length. Strings have both a length and capacity. &str
    // represents a string slice. String literals are slices.
}
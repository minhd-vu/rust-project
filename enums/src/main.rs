fn main() {
    println!("Hello, world!");
    // Here animal is an enum, and it can only be either a Dog or Cat. We can also add
    // data to the enum; here, we're adding the name of the animal. You can put any data
    // inside the enum (along with multiple types). Enums can also have named fields like
    // a struct. Defining these enum types are very similar to defining various types of
    // struct definitions.
    enum Animal {
        Dog(String),
        Cat(String),
        Bat { x: i32, y: i32 },
    }
    // Enums are accessed via namespacing.
    let animal = Animal::Dog(String::from("Fido"));

    // Enums can also have impl blocks.
    impl Animal {
        fn eat(&self) {
            println!("animal is eating!");
        }
    }

    // Rust doesn't have the null feature that other languages have. The Option<T> enum is
    // used to represent the same functionality as null. The enum has the values of Some and None.
    // Some represents that there is some value of type T present in the enum, and None represents
    // something similar to null. Generally this allows us to catch something when something is 
    // expected to not be null, when it actaully is.
}

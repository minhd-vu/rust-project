// Items that are not in the rust prelude must be imported in explicitly.
use std::io;

// Function definition, main is the entry point of the program.
fn main() {
    // println is a macro.
    println!("Guess the number!\nPlease input your guess.");
    // let creates a new variable; variables are immutable by default
    // and mutable variables are specified with the mut keyword.
    // The :: specifies an associated function. An associated function
    // is a function that is implemented on a type, in this case new()
    // is the associated function and String is the type.
    let mut guess = String::new();
    io::stdin()
        // The & indicates that a reference is being passed.
        // References are immutable by default, therefore the mut keyword is needed again.
        .read_line(&mut guess)
        // read_line returns the Result type which is used to encode error handling information.
        // Result has two variants, Ok and Err.
        .expect("Failed to read line");

    println!("You guessed: {guess}")
}

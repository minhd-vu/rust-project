// Items that are not in the rust prelude must be imported in explicitly.
use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
// Another way to condense imports.
use std::{cmp::Ordering, io};

// Function definition, main is the entry point of the program.
fn main() {
    // println is a macro.
    println!("Guess the number!");

    // rand::Rng trail must be imported in order to use this method.
    // thread_rng is a random number generator local to the system.
    // the ..= notation denotes an inclusive range, without the equals ..
    // then the latter number would be not included.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    // let creates a new variable; variables are immutable by default
    // and mutable variables are specified with the mut keyword.
    // The :: specifies an associated function. An associated function
    // is a function that is implemented on a type, in this case new()
    // is the associated function and String is the type.
    let mut guess = String::new();

    // loop is an infinite loop.
    loop {
        println!("Please input your guess.");

        guess.clear();

        io::stdin()
            // The & indicates that a reference is being passed.
            // References are immutable by default, therefore the mut keyword is needed again.
            .read_line(&mut guess)
            // read_line returns the Result type which is used to encode error handling information.
            // Result has two variants, Ok and Err.
            .expect("Failed to read line");

        // shadow the previous name guess so that two different variable names
        // are not needed (guess_str and guess_u32).
        // trim will remove the whitespace such as the trailing \n
        // and parse will convert a string into any other type.
        // match the result to handle the non u32 error.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // _ is a catch all.
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // match expression is made up of arms where each arm is a pattern to match against.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                // break quits the loop so the game ends.
                break;
            }
            Ordering::Greater => println!("Too big!"),
        };
    }
}

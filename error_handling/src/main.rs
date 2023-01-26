use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    // Manually force a unrecoverable error.
    // panic!("crash and burn");

    // Try to access an index out of bounds.
    // To see the backtrace, run:
    // RUST_BACKTRACE=1 cargo run
    let v = vec![1, 2, 3];
    v[99];

    // This is how we would handle errors using Result.
    let greeting_file_result = File::open("hello.txt");
    // Try reading a file and return it if successful, if not, check the error, if it is a not
    // found error, create the file.
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // You can also use closures to avoid the verbose use of match.
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Using unwrap will just call the panic if there is an error.
    let greeting_file = File::open("hello.txt").unwrap();

    // Using expect allows you to define the error message.
    let greeting_file = File::open("hello.txt").expect("could not find hello.txt");
}

// You can return a Result from a function to pass error handling up the stack.
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// This is the same as the above but Rust provides the ? operator as a shorthand.
// What is does is return the Ok value to the expression and keep the program
// running. If it is an Err, then it will return the error from the function. The
// operator can only be used on functions that have the same Result as the thing
// the ? is used on.
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// You can even make the above shorter!
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username);
    Ok(username)
}

// The shortest!
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// The ? operator can also be used on Option<T> values. Here the next()? means
// that there may be a line or no line, and it will return None if there isn't
// one.
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

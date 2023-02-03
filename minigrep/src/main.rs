use minigrep::{run, Config};
use std::{env, process};

// The main function should only handle argument parsing, config setup, calling
// run, and handling errors from run.
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!(
        "Searching for {} in file {}.",
        config.query, config.file_path
    );

    // We don't use unwrap_or_else here because we don't care about the Ok
    // return value of run, only the error.
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

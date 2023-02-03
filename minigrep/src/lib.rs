use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // new handles the parsing of the command line arguments into data.
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // We have to use return here.
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// The unit type () is similar to void. The Box<dyn Error> means that it can
// return any type that implements Error.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");

    // This is the idiomatic way of calling run only for its side effects.
    Ok(())
}

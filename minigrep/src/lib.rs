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

    for line in search(&config.query, &contents) {
        println!("{line}")
    }

    // This is the idiomatic way of calling run only for its side effects.
    Ok(())
}

// Here the lifetimes indicate that the vector should return string slices that
// reference the contents argument rather than the query argument i.e. the data
// returned by search will live as long as the data in the argument contents
// (not query!)
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matches = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // The backslash at the start tells rust to not put a newline at the start.
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

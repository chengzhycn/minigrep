use std::error::Error;
use std::fs;

use clap::ArgMatches;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(opts: &ArgMatches) -> Result<Config, &'static str> {
        if opts.value_of("query").is_none() {
            return Err("query must be specified.");
        }

        if opts.value_of("file").is_none() {
            return Err("file must be specified.");
        }

        let case_insensitive = opts.is_present("case_insensitive");

        Ok(Config {
            query: opts.value_of("query").unwrap().to_string(),
            filename: opts.value_of("file").unwrap().to_string(),
            case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let result = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
pick three.
rust is the best language in the world.";

        assert_eq!(
            vec!["Rust:", "rust is the best language in the world."],
            search_case_insensitive(&query, &contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = Vec::new();
    for s in contents.lines() {
        if s.contains(query) {
            v.push(s);
        }
    }

    v
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = Vec::new();
    let q = query.to_ascii_lowercase();
    for s in contents.lines() {
        if s.to_ascii_lowercase().contains(&q) {
            v.push(s);
        }
    }

    v
}

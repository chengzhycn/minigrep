use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // args check
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
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

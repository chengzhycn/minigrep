use std::process;

use clap::{App, Arg};

use minigrep::Config;

fn main() {
    let matches = App::new("minigrep")
        .author("chengzhycn <chengzhycn@gmail.com>")
        .about("a toy tool likes linux command grep.")
        .version("0.0.1")
        .arg(
            Arg::new("query")
                .required(true)
                .about("query string which need to be searched.")
                .index(1),
        )
        .arg(
            Arg::new("file")
                .required(true)
                .about("file name which need search.")
                .index(2),
        )
        .arg(
            Arg::new("case_insensitive")
                .about("case insensitive search.")
                .short('i'),
        )
        .get_matches();

    // assignment
    let config = Config::new(&matches).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

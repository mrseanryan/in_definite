//! in_definite_cmd command line, mostly as a test harness for the in_definite library.

use std::env;
use std::process;

use in_definite;

/// Get the indefinite article ('a' or 'an') to match the given word. For example: an umbrella, a user.
fn main() {
    println!("= in_definite =");
    println!("===============");

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let result = in_definite::get_a_or_an(&config.word);

    println!("{} {}", result, config.word);
}

struct Config {
    word: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() != 2 {
            let usage = "USAGE: ";
            let usage = usage.to_string() + &args[0];
            let usage = usage + " [word]";

            return Err(usage);
        }
        let word = args[1].clone();
        return Ok(Config { word });
    }
}

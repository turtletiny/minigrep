use minigrep::{search, search_case_insensitive};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(69);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(69);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    // pub inverse: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let flags = parse_flags(args);

        Ok(Self {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
            // inverse: ,
        })
    }
}

struct FlagConfig {
    inverse: bool,
    case_insensitive: bool,
    line_numbers: bool,
}

fn parse_flags(args: &[String]) -> Vec<char> {
    let mut flags = Vec::new();
    let mut i = 1;

    while args[i].as_bytes()[0] == b'-' {
        for j in 1..args[i].len() {
            flags.push(args[i].as_bytes()[j] as char);
        }
        i += 1;
    }

    flags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flag_test() {
        let args = vec![
            String::from("ignore"),
            String::from("-f"),
            String::from("-i"),
            String::from("-zxi"),
            String::from("pattern"),
            String::from("file.txt"),
        ];

        assert_eq!(vec!['f', 'i', 'z', 'x', 'i'], parse_flags(&args))
    }
}

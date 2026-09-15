use minigrep::{search, search_case_insensitive};
use minigrep::{search_inverse, search_inverse_insensitive};
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

    let results = if config.ignore_case && config.inverse {
        search_inverse_insensitive(&config.query, &contents)
    } else if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else if config.inverse {
        search_inverse(&config.query, &contents)
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
    pub inverse: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let argc = args.len();
        if argc < 3 {
            return Err("Not enough arguments");
        }

        let flags = parse_flags(args);
        let (query, file_path) = if flags.is_empty() {
            (args[1].clone(), args[2].clone())
        } else {
            (args[2].clone(), args[3].clone())
        };

        let mut default_config = Config {
            query,
            file_path,
            ignore_case: false,
            inverse: false,
        };

        for c in &flags[..] {
            let flag = match char_to_flag(*c) {
                Some(f) => f,
                None => return Err("Invalid flag"),
            };

            default_config.update(flag);
        }

        Ok(default_config)
    }

    fn update(&mut self, flag: Flag) {
        match flag {
            Flag::i => self.ignore_case = true,
            Flag::v => self.inverse = true,
        }
    }
}

#[allow(non_camel_case_types)]
pub enum Flag {
    i,
    v,
}

pub fn parse_flags(args: &[String]) -> Vec<char> {
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

pub fn char_to_flag(c: char) -> Option<Flag> {
    match c {
        'i' => Some(Flag::i),
        'v' => Some(Flag::v),
        _ => None,
    }
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

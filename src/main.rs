use minigrep::{search, search_case_insensitive};
use minigrep::{search_inverse, search_inverse_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(69);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(69);
    }

    println!("Took: {}s", start_time.elapsed().as_secs_f32());
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

        let (flags, path_idx) = parse_flags_and_query_index(args);
        if path_idx + 2 != argc {
            return Err("pattern or path not provided");
        }

        let query = args[path_idx].clone();
        let file_path = args[path_idx + 1].clone();

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

pub fn parse_flags_and_query_index(args: &[String]) -> (Vec<char>, usize) {
    let mut flags = Vec::new();
    let mut query_idx = 1;

    while args[query_idx].as_bytes()[0] == b'-' {
        for j in 1..args[query_idx].len() {
            flags.push(args[query_idx].as_bytes()[j] as char);
        }
        query_idx += 1;
    }

    (flags, query_idx)
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

        assert_eq!(
            vec!['f', 'i', 'z', 'x', 'i'],
            parse_flags_and_query_index(&args).0
        )
    }
}

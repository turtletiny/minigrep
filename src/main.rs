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

    let config = Config::build(&args).unwrap_or_else(|e| {
        eprintln!("Problem parsing args: {e}");
        process::exit(69);
    });


    if let Err(e) = run(&config) {
        eprintln!("Application error: {e}");
        process::exit(69);
    }

    println!("Took: {}s", start_time.elapsed().as_secs_f32());
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    for f in &config.file_paths {
        let contents = fs::read_to_string(f)?;

        todo!(program insta exits upon seeing invalid file path, rather than moving onto the next file path);

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
    }

    Ok(())
}

struct Config {
    pub query: String,
    pub file_paths: Vec<String>,
    pub ignore_case: bool,
    pub inverse: bool,
}

// impl Config {
//     fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
//         args.next();
//
//         let query = match args.next()?;
//         let file_path = match args.next() {
//             Some(arg) => arg,
//             None => return Err("No file path"),
//         };
//
//         let ignore_case = env::var("IGNORE_CASE").is_ok();
//
//         Ok(Config {
//             query,
//             file_path,
//             ignore_case,
//             inverse: false,
//         })
//     }

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        let arg_count = args.len();
        if arg_count < 3 {
            return Err("Not enough arguments");
        }

        let mut ignore_case = false;
        let mut inverse = false;

        let mut i = 1;
        while i < arg_count && args[i].starts_with('-') {
            match args[i].as_bytes().get(1) {
                Some(b'v') => inverse = true,
                Some(b'i') => ignore_case = true,
                Some(_) => return Err("Invalid flag"),
                None => break,
            }
            i += 1;
        }

        let query = match args.get(i) {
            Some(s) => s.clone(),
            None => return Err("No query found"),
        };
        i += 1;

        let mut file_paths = vec![args[i].clone()];
        i += 1;

        while i < arg_count && !args[i].starts_with('-') {
            file_paths.push(args[i].clone());
            i += 1;
        }

        Ok(Config {
            query,
            file_paths,
            ignore_case,
            inverse,
        })
    }
}

// while args[i].into_bytes().[0] == b'-' {
//     let (flags_vec, query_idx) = parse_flags_and_query_index(args);

// fn idk() {
//     match c {
//         'i' => update_config(),
//         'b' => update_config(),
//         _ => return Err("Invalid flag (--help for more info)");
//     }
// }

// fn parse_input(&)

// fn build(args: &[String]) -> Result<Config, &'static str> {
//     let arg_count = args.len();
//
//     if arg_count < 3 {
//         return Err("Not enough arguments");
//     }
//
//     let (flags, path_idx) = parse_flags_and_query_index(args);
//     if path_idx + 2 != arg_count {
//         return Err("pattern or path not provided");
//     }
//
//     let query = args[path_idx].clone();
//     let file_path = args[path_idx + 1].clone();
//
//     let mut default_config = Config {
//         query,
//         file_path,
//         ignore_case: false,
//         inverse: false,
//     };
//
//     for c in &flags[..] {
//         let flag = match char_to_flag(*c) {
//             Some(f) => f,
//             None => return Err("Invalid flag"),
//         };
//
//         default_config.update(flag);
//     }
//
//     Ok(default_config)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn flag_test() {
//         let args = vec![
//             String::from("ignore"),
//             String::from("-f"),
//             String::from("-i"),
//             String::from("-zxi"),
//             String::from("pattern"),
//             String::from("file.txt"),
//         ];
//
//         // assert_eq!(
//         //     vec!['f', 'i', 'z', 'x', 'i'],
//         //     parse_flags_and_query_index(&args).0
//         // )
//     }
// }

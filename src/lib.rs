use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_paths: Vec<String>,
    pub ignore_case: bool,
    pub inverse: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let arg_count = args.len();
        if arg_count < 3 {
            return Err("Not enough arguments");
        }

        let mut ignore_case = false;
        let mut inverse = false;

        let mut i = 1;
        while i < arg_count && args[i].starts_with('-') {
            for b in args[i].as_bytes().iter().skip(1) {
                match b {
                    b'v' => inverse = true,
                    b'i' => ignore_case = true,
                    _ => return Err("unrecognised flag"),
                }
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
pub fn search<'a>(
    query: &str,
    contents: &'a str,
    config: &Config,
) -> impl Iterator<Item = (usize, &'a str)> {
    let query = if config.ignore_case {
        query.to_lowercase()
    } else {
        query.to_string()
    };
    contents
        .lines()
        .enumerate()
        .filter(move |(_idx, line)| {
            if config.inverse && config.ignore_case {
                !line.to_lowercase().contains(&query)
            } else if config.inverse {
                !line.contains(&query)
            } else if !config.inverse && config.ignore_case {
                line.to_lowercase().contains(&query)
            } else {
                line.contains(&query)
            }
        })
        .map(|(idx, line)| (idx + 1, line))
}

// -i
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// -v
pub fn search_inverse<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if !line.contains(query) {
            results.push(line);
        }
    }

    results
}

// -iv
pub fn search_inverse_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if !line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";
//
//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     }
//
//     #[test]
//     fn inverse() {
//         let query = "bob";
//         let contents = "\
// jeff
// jeffbob
// bobbardly jeff
// hi
// hey
// bObbS";
//         assert_eq!(
//             vec!["jeff", "hi", "hey", "bObbS"],
//             search_inverse(query, contents)
//         );
//     }
// }

use std::error::Error;
use std::fs;
use std::path::Path;

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

    // args is a string slice from [i..], aka the start of all paths to search
    pub fn build_file_paths(&mut self, args: &[String]) {
        for i in args {
            if Path::new(i).is_dir() {
                // walk the dir
            } else {
                self.file_paths.push(i.clone());
            }
        }
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
            let mut filter_res = if config.ignore_case {
                line.to_lowercase().contains(&query)
            } else {
                line.contains(&query)
            };

            if config.inverse {
                filter_res = !filter_res;
            }

            filter_res
        })
        .map(|(idx, line)| (idx + 1, line))
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

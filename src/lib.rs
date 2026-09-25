// const CONFIG_PATH: &str = "config.toml";

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::{fs, process};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_paths: Vec<String>,
    pub ignore_case: bool,
    pub inverse: bool,
    pub show_stat: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let arg_count = args.len();

        if args[1] == "-h" {
            help_msg();
            return Err("u received help");
        }

        if arg_count < 3 {
            return Err("minigrep [FLAGS] [PATTERN] [PATHS]");
        }

        let mut ignore_case = false;
        let mut inverse = false;
        let mut show_stat = false;

        let mut i = 1;
        while i < arg_count && args[i].starts_with('-') {
            for b in args[i].as_bytes().iter().skip(1) {
                match b {
                    b'h' => {
                        help_msg();
                        return Err("");
                    }
                    b'v' => inverse = true,
                    b'i' => ignore_case = true,
                    b's' => show_stat = true,
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
            show_stat,
        })
    }
}

pub fn search<'a>(contents: &'a str, config: &Config) -> impl Iterator<Item = (usize, &'a str)> {
    let query = if config.ignore_case {
        config.query.to_lowercase()
    } else {
        config.query.to_string()
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


// NOTE: currently only assumes 1 match in a line 
pub fn highlight_line(line: &String, query: &String) -> String {
    let query_len = query.chars().count();
    let query_first = query.chars().next().unwrap();
    let mut res = String::with_capacity(line.len());
    let last_pushed = 0;

    for (i, c) in line.char_indices() {
        if c == query_first && line[i..i+query_len] == *query {
            res.push_str(
                format!(
                    "{}\x1b[1;33m{query}\x1b[0m{}",
                    &line[last_pushed..(i)],
                    &line[i + query_len..]
                )
                .as_str(),
            );
            break;
        }
    }

    res
}

pub fn format_metadata<P: AsRef<Path>>(path: &P) -> std::io::Result<String> {
    let m = fs::metadata(path)?;

    let size = m.len();
    let file_type = m.file_type();
    let file_type = if file_type.is_dir() {
        " "
    } else if file_type.is_file() {
        "󰈔 "
    } else if file_type.is_symlink() {
        " "
    } else {
        "other"
    };

    // bitwise op to get "3 permission digits"
    let permission = m.permissions().mode() & 0o777;

    Ok(format!(
        "{} {file_type}{size}b {permission:o}",
        path.as_ref().display()
    ))
}

pub fn help_msg() {
    println!("USAGE:");
    println!("grep [OPTIONS] PATTERN [PATHS]");
    println!();

    println!("SEARCH OPTIONS:");
    println!("  -i    Case insensitive search");
    println!("  -v    Invert matching (match lines without pattern");
    println!();

    println!("STAT OPTIONS: ");
    println!("  -s    show file statistics, similar to ls -L");
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

    use super::*;

    // #[test]
    // fn test_metadata() {
    //     let path = PathBuf::from_str("smol.txt").unwrap();
    //     match format_metadata(&path) {
    //         Ok(s) => println!("{s}"),
    //         _ => println!("no"),
    //     }
    // }

    #[test]
    fn test_highlight() {
        let query = String::from("bobly1");
        let line = String::from("wbobly1IDK!");
        // let res = cmpstr(&query, &line);
        // println!("{res}");
        println!("{}", highlight_line(&line, &query));
    }

    // #[test]
    // fn idk() {
    // }
}

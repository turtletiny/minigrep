// const CONFIG_PATH: &str = "config.toml";

use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::fs;

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

pub fn search(contents: &str, config: &Config) -> impl Iterator<Item = (usize, String)> {
    let query = if config.ignore_case {
        config.query.to_lowercase()
    } else {
        config.query.clone()
    };
    let query_is_empty = query.is_empty();

    contents
        .lines()
        .enumerate()
        .filter(move |(_idx, line)| {
            let mut matches = if config.ignore_case {
                line.to_lowercase().contains(&query)
            } else {
                line.contains(&query)
            };

            if config.inverse {
                matches = !matches;
            }

            matches
        })
        .map(move |(idx, line)| {
            let highlighted = if config.inverse || query_is_empty {
                line.to_owned()
            } else {
                highlight_line(line, &config.query, config.ignore_case)
            };

            (idx + 1, highlighted)
        })
}


pub fn highlight_line(line: &str, query: &str, ignore_case: bool) -> String {
    if query.is_empty() {
        return line.to_owned();
    }

    let query_chars = query.chars().count();
    let query_for_match = query.to_lowercase();
    let mut result = String::with_capacity(line.len());
    let mut last_end = 0;

    for (start, _) in line.char_indices() {
        let Some(end) = line[start..]
            .char_indices()
            .nth(query_chars)
            .map(|(offset, _)| start + offset)
            .or_else(|| {
                (line[start..].chars().count() == query_chars)
                    .then_some(line.len())
            })
        else {
            break;
        };

        let candidate = &line[start..end];
        let matches = if ignore_case {
            candidate.to_lowercase() == query_for_match
        } else {
            candidate == query
        };

        if matches {
            result.push_str(&line[last_end..start]);
            result.push_str("\x1b[1;33m");
            result.push_str(candidate);
            result.push_str("\x1b[0m");
            last_end = end;
        }
    }

    result.push_str(&line[last_end..]);
    result
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
        assert_eq!(
            highlight_line(&line, &query, false),
            "w\x1b[1;33mbobly1\x1b[0mIDK!"
        );
    }

    #[test]
    fn search_highlights_each_match() {
        let config = Config {
            query: "rust".to_owned(),
            file_paths: Vec::new(),
            ignore_case: true,
            inverse: false,
            show_stat: false,
        };

        let results: Vec<_> = search("Rust is RUST\nno match", &config).collect();

        assert_eq!(
            results,
            vec![(
                1,
                "\x1b[1;33mRust\x1b[0m is \x1b[1;33mRUST\x1b[0m".to_owned()
            )]
        );
    }

    // #[test]
    // fn idk() {
    // }
}

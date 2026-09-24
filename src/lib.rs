// const CONFIG_PATH: &str = "config.toml";

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

#[derive(Debug)]
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


pub fn format_metadata<P: AsRef<Path>>(path: &P) -> std::io::Result<String>{
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

    Ok(format!("{} {file_type}{size}b {permission:o}", path.as_ref().display()))
}


#[cfg(test)]
mod test {

    use std::str::FromStr;

use super::*;

    #[test]
    fn test_metadata() {
        let path = PathBuf::from_str("smol.txt").unwrap();
        match format_metadata(&path) {
            Ok(s) => println!("{s}"),
            _ => println!("no")
        }
    }




}

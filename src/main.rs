use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;
use std::time::Instant;

use minigrep::{Config, search};

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
        let path = Path::new(f);

        if path.is_dir() {
            walk_dir(f, config);
        } else {
            let contents = match fs::read_to_string(f) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Application error: {e}");
                    continue;
                }
            };

            let iter = search(&config.query, &contents, config);
            for (idx, line) in iter {
                println!("{idx}: {line}");
            }
        }
    }

    Ok(())
}

fn grep_1_file(path: &Path, config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;

    let iter = search(&config.query, &contents, config);
    for (idx, line) in iter {
        println!("{idx}: {line}");
    }

    Ok(())
}

fn walk_dir(dir: &String, config: &Config) -> std::io::Result<()> {
    let entries = fs::read_dir(dir)?;

    for path in entries {
        let e = path?;
        if !e.file_type()?.is_dir() {
            println!("{e:?}\n");
            grep_1_file(&e.path(), config);
        }
    }

    Ok(())
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    // fn test_walk_dir() {
    //     walk_dir(&".".to_string());
    // }
    #[test]
    // fn test_grep_1_file() {
    //     let config = Config {
    //         query: "pig".to_string(),
    //         file_paths: Vec::new(),
    //         ignore_case: true,
    //         inverse: false,
    //     };
    //     grep_1_file(&"smol.txt".to_string(), &config).unwrap();
    // }
    fn test_walk_dir() {
        let config = Config {
            query: "pig".to_string(),
            file_paths: Vec::new(),
            ignore_case: true,
            inverse: false,
        };
        walk_dir(&".".to_string(), &config);
    }
}

// if its a dir, ignore

/*
STEP 1: IMPLEMENT WITH THE ASSUMPTION THAT A DIR MUST END IN "/"

"file": can be either a single file, or a directory
"dir_name"
if it ends in /, its a dir
if not, it should be a file
if no file with that name is found,  then check for a matching dir
if a matchign dir is found, treat it as a dir, else ERR
 */

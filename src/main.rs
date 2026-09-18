use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::time::Instant;


use minigrep::{search, Config};


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
        let contents = match fs::read_to_string(f) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Application error: {e}");
                continue;
            }
        };

        let iter = search(&config.query, &contents, &config);
        for (idx, line) in iter {
            println!("{idx}: {line}");
        }
    }

    Ok(())
}


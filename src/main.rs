use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let config = parse_config(&args);

    println!("searching for {}", config.query);
    println!("in file {}", config.file_path);

    let contents =
        fs::read_to_string(&config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    Config {
        query: args[1].clone(),
        file_path: args[2].clone(),
    }
}

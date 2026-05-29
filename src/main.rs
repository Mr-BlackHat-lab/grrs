use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    // Handle file open errors using match
    let file = match File::open(&args.path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open file '{}': {}", args.path.display(), err);
            return;
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        // Handle line read errors using match
        match line {
            Ok(content) => {
                if content.contains(&args.pattern) {
                    println!("{}", content);
                }
            }
            Err(err) => {
                eprintln!("Failed to read line: {}", err);
            }
        }
    }
}

// In main.rs
mod file_ops;
mod text_analysis;
mod cli;
use std::process;
fn main() {
    let config = cli::parse_args().unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Processing file: {}", config.filename);
    
    let content = file_ops::read_file(&config.filename).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    });
    
    match config.operation {
        cli::Operation::CountWords => {
            let count = text_analysis::count_words(&content);
            println!("Word count: {}", count);
        },
        cli::Operation::CountLines => {
            let count = text_analysis::count_lines(&content);
            println!("Line count: {}", count);
        },
        cli::Operation::CountChars => {
            let count = text_analysis::count_chars(&content);
            println!("Character count: {}", count);
        },
    }
}

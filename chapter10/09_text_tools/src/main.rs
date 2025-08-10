use std::fs;
use std::process;

mod cli;
mod text_analysis;

fn main() {
    let config = cli::parse_args();
    
    let content = match fs::read_to_string(&config.filename) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading file '{}': {}", config.filename, err);
            process::exit(1);
        }
    };
    
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
        cli::Operation::CountPattern(pattern) => {
            match text_analysis::count_pattern(&content, &pattern) {
                Ok(count) => println!("Pattern '{}' count: {}", pattern, count),
                Err(err) => {
                    eprintln!("Error with pattern '{}': {}", pattern, err);
                    process::exit(1);
                }
            }
        },
    }
}

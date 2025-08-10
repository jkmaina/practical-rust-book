use std::env;

pub struct Config {
    pub filename: String,
    pub operation: Operation,
}

pub enum Operation {
    CountWords,
    CountLines,
    CountChars,
    CountPattern(String),
}

pub fn parse_args() -> Config {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <filename> <operation> [pattern]", args[0]);
        std::process::exit(1);
    }
    
    let filename = args[1].clone();
    
    let operation = match args[2].as_str() {
        "words" => Operation::CountWords,
        "lines" => Operation::CountLines,
        "chars" => Operation::CountChars,
        "pattern" => {
            if args.len() < 4 {
                eprintln!("Pattern operation requires a pattern argument");
                std::process::exit(1);
            }
            Operation::CountPattern(args[3].clone())
        },
        _ => {
            eprintln!("Unknown operation: {}", args[2]);
            std::process::exit(1);
        }
    };
    
    Config { filename, operation }
}

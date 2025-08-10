// In cli.rs
use std::env;
pub struct Config {
    pub filename: String,
    pub operation: Operation,
}
pub enum Operation {
    CountWords,
    CountLines,
    CountChars,
}
pub fn parse_args() -> Result<Config, &'static str> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    
    let filename = args[1].clone();
    
    let operation = match args[2].as_str() {
        "words" => Operation::CountWords,
        "lines" => Operation::CountLines,
        "chars" => Operation::CountChars,
        _ => return Err("unknown operation"),
    };
    
    Ok(Config { filename, operation })
} 
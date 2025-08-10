use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
/// A simple word count tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to count
    #[arg(name = "FILE")]
    file: PathBuf,
    /// Count lines
    #[arg(short = 'l', long)]
    lines: bool,
    /// Count words
    #[arg(short = 'w', long)]
    words: bool,
    /// Count characters
    #[arg(short = 'c', long)]
    chars: bool,
}
fn main() -> Result<()> {
    let args = Args::parse();
    // If no specific counts are requested, show all
    let show_all = !args.lines && !args.words && !args.chars;
    // Open the file
    let file = File::open(&args.file)
        .with_context(|| format!("Failed to open file: {}", args.file.display()))?;
    let reader = BufReader::new(file);
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;
    // Process the file
    for line in reader.lines() {
        let line = line.context("Failed to read line")?;
        line_count += 1;
        word_count += line.split_whitespace().count();
        char_count += line.chars().count();
    }
    // Print the results
    let mut output = String::new();
    if args.lines || show_all {
        output.push_str(&format!("{} ", line_count));
    }
    if args.words || show_all {
        output.push_str(&format!("{} ", word_count));
    }
    if args.chars || show_all {
        output.push_str(&format!("{} ", char_count));
    }
    output.push_str(&args.file.display().to_string());
    println!("{}", output);
    Ok(())
}
 

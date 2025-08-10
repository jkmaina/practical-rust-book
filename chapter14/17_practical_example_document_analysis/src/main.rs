// Document analysis example demonstrating HashMap, iterators, and string processing
// Counts word frequencies, finds most common words, and calculates statistics
// Shows practical use of collections and functional programming patterns

use std::collections::HashMap;

fn analyze_document(text: &str) -> HashMap<String, usize> {
    let mut word_count = HashMap::new();
    
    // Count word frequencies
    for word in text.split_whitespace() {
        // Convert to lowercase and remove punctuation
        let word = word.to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();
        
        if !word.is_empty() {
            *word_count.entry(word).or_insert(0) += 1;
        }
    }
    
    word_count
}
fn main() {
    let text = "This is a sample text. This text is meant to be analyzed. \
                The analysis will count how many times each word appears. \
                Text analysis is a common task in natural language processing.";
    
    println!("Analyzing text: {}", text);
    println!();
    
    let word_counts = analyze_document(text);
    
    // Find the most common words
    let mut words: Vec<(&String, &usize)> = word_counts.iter().collect();
    words.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("Word frequency analysis:");
    for (word, count) in words.iter().take(5) {
        println!("{}: {}", word, count);
    }
    
    // Calculate statistics
    let total_words: usize = word_counts.values().sum();
    let unique_words = word_counts.len();
    let average_frequency = total_words as f64 / unique_words as f64;
    
    println!("\nStatistics:");
    println!("Total words: {}", total_words);
    println!("Unique words: {}", unique_words);
    println!("Average frequency: {:.2}", average_frequency);
}
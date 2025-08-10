// This program demonstrates a practical example of using lifetimes with structs in Rust.
// The `TextAnalyzer` struct holds a reference to a string slice and provides methods
// to analyze the text, such as counting words and characters. The lifetime parameter `'a`
// ensures that the struct cannot outlive the data it references.

struct TextAnalyzer<'a> {
    text: &'a str,
}

impl<'a> TextAnalyzer<'a> {
    // Creates a new TextAnalyzer with a reference to the given text
    fn new(text: &'a str) -> TextAnalyzer<'a> {
        TextAnalyzer { text }
    }
    
    // Counts the number of words in the text
    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
    
    // Counts the number of characters in the text
    fn character_count(&self) -> usize {
        self.text.chars().count()
    }
    // Returns the first word in the text
    fn first_word(&self) -> &str {
        match self.text.find(' ') {
            Some(pos) => &self.text[0..pos],
            None => self.text,
        }
    }    
}

    // Returns a reference to the longest word in the text
    fn analyzer_with_more_words<'a>(a1: &'a TextAnalyzer<'a>, a2: &'a TextAnalyzer<'a>) -> &'a TextAnalyzer<'a> {
    if a1.word_count() > a2.word_count() {
        a1
    } else {
        a2
    }
}

fn main() {
    // Create a String to analyze
    let text = String::from("Hello, world! This is a simple text for analysis.");
    
    // Create a TextAnalyzer that borrows the text
    let analyzer = TextAnalyzer::new(&text);
    
    // Print the word and character counts
    println!("Word count: {}", analyzer.word_count());
    println!("Character count: {}", analyzer.character_count());
    // Print the first word
    println!("First word: {}", analyzer.first_word());

    let text1 = String::from("Hello, world!");
    let text2 = String::from("This is a longer text with more words.");
    
    let analyzer1 = TextAnalyzer::new(&text1);
    let analyzer2 = TextAnalyzer::new(&text2);
    
    let analyzer_with_more = analyzer_with_more_words(&analyzer1, &analyzer2);
    
    println!("The analyzer with more words has {} words.", analyzer_with_more.word_count());


}


// In text_analysis.rs
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}
pub fn count_lines(text: &str) -> usize {
    text.lines().count()
}
pub fn count_chars(text: &str) -> usize {
    text.chars().count()
}

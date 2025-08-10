use regex::Regex;
pub fn count_pattern(text: &str, pattern: &str) -> Result<usize, regex::Error> {
    let regex = Regex::new(pattern)?;
    Ok(regex.find_iter(text).count())
}
pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}
pub fn count_lines(text: &str) -> usize {
    text.lines().count()
}
pub fn count_chars(text: &str) -> usize {
    text.chars().count()
}

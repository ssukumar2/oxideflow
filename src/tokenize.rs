//! Tokenize log lines and compute word frequencies.

use crate::parser::LogLine;

#[allow(dead_code)]
pub fn word_counts(lines: &[LogLine]) -> std::collections::HashMap<String, usize> {
    let re = regex::Regex::new(r"[A-Za-z]{3,}").unwrap();
    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for line in lines {
        for m in re.find_iter(&line.raw) {
            let word = m.as_str().to_lowercase();
            *counts.entry(word).or_insert(0) += 1;
        }
    }
    counts
}

#[allow(dead_code)]
pub fn top_words(lines: &[LogLine], n: usize) -> Vec<(String, usize)> {
    let mut v: Vec<(String, usize)> = word_counts(lines).into_iter().collect();
    v.sort_by_key(|b| std::cmp::Reverse(b.1));
    v.truncate(n);
    v
}

#[allow(dead_code)]
pub fn unique_word_count(lines: &[LogLine]) -> usize {
    word_counts(lines).len()
}

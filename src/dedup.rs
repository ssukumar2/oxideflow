use std::collections::HashMap;
use crate::parser::LogLine;

pub struct Deduplicator {
    window: usize,
    seen: HashMap<String, usize>,
}

impl Deduplicator {
    pub fn new(window: usize) -> Self {
        Self { window, seen: HashMap::new() }
    }

    pub fn process<'a>(&mut self, entries: &'a [LogLine]) -> Vec<&'a LogLine> {
        let mut result = Vec::new();
        self.seen.clear();

        for entry in entries {
            let count = self.seen.entry(entry.raw.clone()).or_insert(0);
            *count += 1;

            if *count <= self.window {
                result.push(entry);
            }
        }

        result
    }

    pub fn duplicates_found(&self) -> Vec<(String, usize)> {
        self.seen.iter()
            .filter(|(_, &count)| count > 1)
            .map(|(msg, &count)| (msg.clone(), count))
            .collect()
    }

    pub fn total_suppressed(&self) -> usize {
        self.seen.values()
            .filter(|&&count| count > self.window)
            .map(|&count| count - self.window)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::LogLine;

    #[test]
    fn test_dedup_suppresses_repeats() {
        let lines: Vec<LogLine> = (0..5).map(|i| LogLine {
            line_number: i + 1,
            level: Some("INFO".to_string()),
            raw: "same message".to_string(),
        }).collect();

        let mut dedup = Deduplicator::new(1);
        let result = dedup.process(&lines);
        assert_eq!(result.len(), 1);
        assert_eq!(dedup.total_suppressed(), 4);
    }

    #[test]
    fn test_dedup_keeps_unique() {
        let lines: Vec<LogLine> = (0..3).map(|i| LogLine {
            line_number: i + 1,
            level: Some("INFO".to_string()),
            raw: format!("message {}", i),
        }).collect();

        let mut dedup = Deduplicator::new(1);
        let result = dedup.process(&lines);
        assert_eq!(result.len(), 3);
        assert_eq!(dedup.total_suppressed(), 0);
    }
}

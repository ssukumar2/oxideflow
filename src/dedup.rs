use crate::parser::LogLine;
use std::collections::HashMap;

pub struct Deduplicator {
    window: usize,
    seen: HashMap<String, usize>,
}

impl Deduplicator {
    pub fn new(window: usize) -> Self {
        Self {
            window,
            seen: HashMap::new(),
        }
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
        self.seen
            .iter()
            .filter(|(_, &count)| count > 1)
            .map(|(msg, &count)| (msg.clone(), count))
            .collect()
    }

    pub fn total_suppressed(&self) -> usize {
        self.seen
            .values()
            .filter(|&&count| count > self.window)
            .map(|&count| count - self.window)
            .sum()
    }
}

pub fn top_errors(lines: &[crate::parser::LogLine], n: usize) -> Vec<(String, usize)> {
    use std::collections::HashMap;
    let mut counts: HashMap<String, usize> = HashMap::new();
    for line in lines {
        if line
            .level
            .as_deref()
            .map(|s| s.eq_ignore_ascii_case("ERROR"))
            .unwrap_or(false)
        {
            *counts.entry(line.raw.clone()).or_insert(0) += 1;
        }
    }
    let mut v: Vec<_> = counts.into_iter().collect();
    v.sort_by_key(|b| std::cmp::Reverse(b.1));
    v.truncate(n);
    v
}

/// Normalize a line by replacing numbers and UUIDs with placeholders,
/// so "request 1234 took 50ms" and "request 5678 took 80ms" collapse.
#[allow(dead_code)]
pub fn normalize_for_dedup(raw: &str) -> String {
    let re_num = regex::Regex::new(r"\d+").unwrap();
    let re_uuid = regex::Regex::new(
        r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}",
    )
    .unwrap();
    let stage1 = re_uuid.replace_all(raw, "<UUID>").to_string();
    re_num.replace_all(&stage1, "<N>").to_string()
}

/// Count distinct normalized message patterns.
#[allow(dead_code)]
pub fn fuzzy_pattern_counts(
    lines: &[crate::parser::LogLine],
) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for line in lines {
        let pattern = normalize_for_dedup(&line.raw);
        *counts.entry(pattern).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::LogLine;

    #[test]
    fn test_dedup_suppresses_repeats() {
        let lines: Vec<LogLine> = (0..5)
            .map(|i| LogLine {
                line_number: i + 1,
                level: Some("INFO".to_string()),
                raw: "same message".to_string(),
            })
            .collect();

        let mut dedup = Deduplicator::new(1);
        let result = dedup.process(&lines);
        assert_eq!(result.len(), 1);
        assert_eq!(dedup.total_suppressed(), 4);
    }

    #[test]
    fn test_dedup_keeps_unique() {
        let lines: Vec<LogLine> = (0..3)
            .map(|i| LogLine {
                line_number: i + 1,
                level: Some("INFO".to_string()),
                raw: format!("message {}", i),
            })
            .collect();

        let mut dedup = Deduplicator::new(1);
        let result = dedup.process(&lines);
        assert_eq!(result.len(), 3);
        assert_eq!(dedup.total_suppressed(), 0);
    }
}

use crate::parser::LogLine;
use std::collections::HashMap;

pub fn count_levels(lines: &[LogLine]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for line in lines {
        if let Some(level) = &line.level {
            *counts.entry(level.to_uppercase()).or_insert(0) += 1;
        } else {
            *counts.entry("UNKNOWN".to_string()).or_insert(0) += 1;
        }
    }
    counts
}

pub fn total_lines(lines: &[LogLine]) -> usize {
    lines.len()
}

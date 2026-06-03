//! Detect bursts of identical messages within short time windows.

use crate::parser::LogLine;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Burst {
    pub message: String,
    pub count: usize,
    pub first_line: usize,
    pub last_line: usize,
}

/// Detect bursts where the same raw message appears `min_count` or more times
/// within `window` consecutive lines.
#[allow(dead_code)]
pub fn detect_bursts(lines: &[LogLine], window: usize, min_count: usize) -> Vec<Burst> {
    let mut bursts: Vec<Burst> = Vec::new();
    let mut tracker: HashMap<String, Vec<usize>> = HashMap::new();

    for line in lines {
        let entry = tracker.entry(line.raw.clone()).or_default();
        entry.push(line.line_number);
        entry.retain(|&n| line.line_number - n <= window);
        if entry.len() >= min_count {
            if let Some(existing) = bursts.iter_mut().find(|b| b.message == line.raw) {
                existing.count = entry.len();
                existing.last_line = line.line_number;
            } else {
                bursts.push(Burst {
                    message: line.raw.clone(),
                    count: entry.len(),
                    first_line: *entry.first().unwrap(),
                    last_line: line.line_number,
                });
            }
        }
    }
    bursts
}

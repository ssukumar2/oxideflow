//! Estimate redundancy in log content.

use crate::parser::LogLine;

/// Ratio of unique raw lines to total lines. Lower = more repetition.
#[allow(dead_code)]
pub fn uniqueness_ratio(lines: &[LogLine]) -> f64 {
    if lines.is_empty() {
        return 0.0;
    }
    let unique: std::collections::HashSet<&str> = lines.iter().map(|l| l.raw.as_str()).collect();
    unique.len() as f64 / lines.len() as f64
}

/// Estimated bytes saved if duplicate lines were collapsed.
#[allow(dead_code)]
pub fn estimated_savings_bytes(lines: &[LogLine]) -> usize {
    let mut seen: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    let mut total = 0usize;
    for line in lines {
        *seen.entry(line.raw.as_str()).or_insert(0) += 1;
        total += line.raw.len();
    }
    let unique_bytes: usize = seen.keys().map(|s| s.len()).sum();
    total.saturating_sub(unique_bytes)
}

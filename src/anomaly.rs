//! Detect unusual spikes in error frequency over time buckets.

use crate::parser::LogLine;

/// Detect hours where ERROR count exceeds `threshold_multiplier` times the average.
/// Returns (hour_prefix, error_count) pairs for anomalous buckets.
#[allow(dead_code)]
pub fn detect_error_spikes(lines: &[LogLine], threshold_multiplier: f64) -> Vec<(String, usize)> {
    let re = regex::Regex::new(r"(\d{4}-\d{2}-\d{2}[T ]\d{2})").unwrap();
    let mut hourly: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for line in lines {
        let is_error = line
            .level
            .as_deref()
            .map(|s| s.eq_ignore_ascii_case("ERROR"))
            .unwrap_or(false);
        if !is_error {
            continue;
        }
        if let Some(m) = re.find(&line.raw) {
            *hourly.entry(m.as_str().to_string()).or_insert(0) += 1;
        }
    }
    if hourly.is_empty() {
        return Vec::new();
    }
    let total: usize = hourly.values().sum();
    let avg = total as f64 / hourly.len() as f64;
    let threshold = avg * threshold_multiplier;
    let mut spikes: Vec<(String, usize)> = hourly
        .into_iter()
        .filter(|(_, c)| (*c as f64) > threshold)
        .collect();
    spikes.sort_by_key(|b| std::cmp::Reverse(b.1));
    spikes
}

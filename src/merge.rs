//! Merge log lines from multiple sources, sorted by timestamp prefix.

use crate::parser::LogLine;

/// Merge multiple slices of LogLine into one, sorted by the timestamp
/// found at the start of each line's `raw` field. Lines without
/// recognizable timestamps appear at the end in original order.
#[allow(dead_code)]
pub fn merge_by_timestamp(sources: &[&[LogLine]]) -> Vec<LogLine> {
    let ts_re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}").unwrap();
    let mut all: Vec<LogLine> = sources.iter().flat_map(|s| s.iter().cloned()).collect();
    all.sort_by(|a, b| {
        let ta = ts_re.find(&a.raw).map(|m| m.as_str());
        let tb = ts_re.find(&b.raw).map(|m| m.as_str());
        match (ta, tb) {
            (Some(x), Some(y)) => x.cmp(y),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
    all
}

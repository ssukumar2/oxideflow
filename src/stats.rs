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

/// Percentage of lines that are at ERROR level (0.0 to 100.0).
#[allow(dead_code)]
pub fn error_rate(lines: &[crate::parser::LogLine]) -> f64 {
    if lines.is_empty() {
        return 0.0;
    }
    let errors = lines
        .iter()
        .filter(|l| {
            l.level
                .as_deref()
                .map(|s| s.eq_ignore_ascii_case("ERROR"))
                .unwrap_or(false)
        })
        .count();
    (errors as f64 / lines.len() as f64) * 100.0
}

/// Count lines per hour bucket based on YYYY-MM-DD HH timestamp prefix.
/// Returns sorted (hour_prefix, count) pairs.
#[allow(dead_code)]
pub fn lines_per_hour(lines: &[crate::parser::LogLine]) -> Vec<(String, usize)> {
    let re = regex::Regex::new(r"(\d{4}-\d{2}-\d{2}[T ]\d{2})").unwrap();
    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for line in lines {
        if let Some(m) = re.find(&line.raw) {
            *counts.entry(m.as_str().to_string()).or_insert(0) += 1;
        }
    }
    let mut v: Vec<_> = counts.into_iter().collect();
    v.sort_by(|a, b| a.0.cmp(&b.0));
    v
}

/// Return the longest log line by raw byte length, or None if empty.
#[allow(dead_code)]
pub fn longest_line(lines: &[crate::parser::LogLine]) -> Option<&crate::parser::LogLine> {
    lines.iter().max_by_key(|l| l.raw.len())
}

/// Return the shortest non-blank log line, or None.
#[allow(dead_code)]
pub fn shortest_line(lines: &[crate::parser::LogLine]) -> Option<&crate::parser::LogLine> {
    lines
        .iter()
        .filter(|l| !l.raw.trim().is_empty())
        .min_by_key(|l| l.raw.len())
}

/// Calculate lines per second between first and last timestamp in the slice.
/// Returns 0.0 if no valid timestamps are found.
#[allow(dead_code)]
pub fn throughput_per_sec(lines: &[crate::parser::LogLine]) -> f64 {
    let re = regex::Regex::new(r"(\d{2}):(\d{2}):(\d{2})").unwrap();
    let mut times: Vec<u64> = Vec::new();
    for l in lines {
        if let Some(c) = re.captures(&l.raw) {
            let h: u64 = c[1].parse().unwrap_or(0);
            let m: u64 = c[2].parse().unwrap_or(0);
            let s: u64 = c[3].parse().unwrap_or(0);
            times.push(h * 3600 + m * 60 + s);
        }
    }
    if times.len() < 2 {
        return 0.0;
    }
    let span = times.iter().max().unwrap() - times.iter().min().unwrap();
    if span == 0 {
        return 0.0;
    }
    lines.len() as f64 / span as f64
}

/// Percentage breakdown by level. Returns (level, percentage) pairs sorted descending.
#[allow(dead_code)]
pub fn level_percentages(lines: &[crate::parser::LogLine]) -> Vec<(String, f64)> {
    if lines.is_empty() {
        return Vec::new();
    }
    let mut counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    for l in lines {
        let key = l
            .level
            .clone()
            .unwrap_or_else(|| "UNKNOWN".to_string())
            .to_uppercase();
        *counts.entry(key).or_insert(0) += 1;
    }
    let total = lines.len() as f64;
    let mut v: Vec<(String, f64)> = counts
        .into_iter()
        .map(|(k, v)| (k, (v as f64 / total) * 100.0))
        .collect();
    v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    v
}

/// Total Unicode character count (not bytes) across all lines.
#[allow(dead_code)]
pub fn total_chars(lines: &[crate::parser::LogLine]) -> usize {
    lines.iter().map(|l| l.raw.chars().count()).sum()
}

/// Count lines containing any non-ASCII character.
#[allow(dead_code)]
pub fn non_ascii_line_count(lines: &[crate::parser::LogLine]) -> usize {
    lines.iter().filter(|l| !l.raw.is_ascii()).count()
}

/// Count transitions between consecutive log levels.
/// Returns ((from_level, to_level), count) pairs sorted descending by count.
#[allow(dead_code)]
pub fn level_transitions(lines: &[crate::parser::LogLine]) -> Vec<((String, String), usize)> {
    let mut counts: std::collections::HashMap<(String, String), usize> =
        std::collections::HashMap::new();
    for pair in lines.windows(2) {
        let a = pair[0]
            .level
            .clone()
            .unwrap_or_else(|| "UNKNOWN".to_string())
            .to_uppercase();
        let b = pair[1]
            .level
            .clone()
            .unwrap_or_else(|| "UNKNOWN".to_string())
            .to_uppercase();
        *counts.entry((a, b)).or_insert(0) += 1;
    }
    let mut v: Vec<_> = counts.into_iter().collect();
    v.sort_by_key(|b| std::cmp::Reverse(b.1));
    v
}

/// Compute the p-th percentile of line lengths (p in 0.0..=100.0).
#[allow(dead_code)]
pub fn line_length_percentile(lines: &[crate::parser::LogLine], p: f64) -> usize {
    if lines.is_empty() {
        return 0;
    }
    let mut lens: Vec<usize> = lines.iter().map(|l| l.raw.len()).collect();
    lens.sort_unstable();
    let idx = ((p / 100.0) * (lens.len() - 1) as f64).round() as usize;
    lens[idx.min(lens.len() - 1)]
}

/// Standard p50/p90/p99 in one call.
#[allow(dead_code)]
pub fn length_quantiles(lines: &[crate::parser::LogLine]) -> (usize, usize, usize) {
    (
        line_length_percentile(lines, 50.0),
        line_length_percentile(lines, 90.0),
        line_length_percentile(lines, 99.0),
    )
}

/// GROUP BY level, returning (level, count, total_bytes) tuples.
#[allow(dead_code)]
pub fn group_by_level(lines: &[crate::parser::LogLine]) -> Vec<(String, usize, usize)> {
    let mut groups: std::collections::HashMap<String, (usize, usize)> =
        std::collections::HashMap::new();
    for l in lines {
        let key = l
            .level
            .clone()
            .unwrap_or_else(|| "UNKNOWN".to_string())
            .to_uppercase();
        let entry = groups.entry(key).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += l.raw.len();
    }
    let mut v: Vec<(String, usize, usize)> = groups
        .into_iter()
        .map(|(k, (count, bytes))| (k, count, bytes))
        .collect();
    v.sort_by_key(|b| std::cmp::Reverse(b.1));
    v
}

/// Return the sorted list of distinct level values found.
#[allow(dead_code)]
pub fn distinct_levels(lines: &[crate::parser::LogLine]) -> Vec<String> {
    let mut set: std::collections::HashSet<String> = std::collections::HashSet::new();
    for l in lines {
        if let Some(lvl) = &l.level {
            set.insert(lvl.to_uppercase());
        }
    }
    let mut v: Vec<String> = set.into_iter().collect();
    v.sort();
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::LogLine;

    fn mk(n: usize, level: Option<&str>) -> LogLine {
        LogLine {
            line_number: n,
            level: level.map(|s| s.to_string()),
            raw: format!("line {}", n),
        }
    }

    #[test]
    fn counts_by_level() {
        let lines = vec![
            mk(1, Some("ERROR")),
            mk(2, Some("error")),
            mk(3, Some("INFO")),
            mk(4, None),
        ];
        let c = count_levels(&lines);
        assert_eq!(c.get("ERROR"), Some(&2));
        assert_eq!(c.get("INFO"), Some(&1));
        assert_eq!(c.get("UNKNOWN"), Some(&1));
    }

    #[test]
    fn total_counts_all() {
        let lines = vec![mk(1, None), mk(2, Some("INFO"))];
        assert_eq!(total_lines(&lines), 2);
    }
}

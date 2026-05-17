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

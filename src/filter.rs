//! Filtering and summarizing log lines.

use crate::parser::LogLine;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;
///use std::cmp::Reverse;

#[derive(Debug, Error)]
pub enum FilterError {
    #[error("invalid regex pattern: {0}")]
    InvalidRegex(#[from] regex::Error),
}

/// Apply level and pattern filters. Both are optional.
/// If a filter is None, that criterion is not applied.
pub fn apply(
    lines: &[LogLine],
    level: Option<&str>,
    pattern: Option<&str>,
) -> Result<Vec<LogLine>, FilterError> {
    let level_upper = level.map(|s| s.to_uppercase());
    let re = pattern.map(Regex::new).transpose()?;

    let out: Vec<LogLine> = lines
        .iter()
        .filter(|line| {
            if let Some(ref lvl) = level_upper {
                if line.level.as_deref() != Some(lvl.as_str()) {
                    return false;
                }
            }
            if let Some(ref re) = re {
                if !re.is_match(&line.raw) {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    Ok(out)
}

/// Filter lines that contain a timestamp matching a simple prefix.
/// For example, prefix "2026-04-16 10:00:1" matches lines from 10:00:10 to 10:00:19.
pub fn filter_by_time_prefix(lines: &[LogLine], prefix: &str) -> Vec<LogLine> {
    lines
        .iter()
        .filter(|line| line.raw.contains(prefix))
        .cloned()
        .collect()
}

#[derive(Debug, Serialize)]
pub struct LogStats {
    pub total_lines: usize,
    pub by_level: HashMap<String, usize>,
    pub lines_without_level: usize,
    pub top_repeated: Vec<(String, usize)>,
}

pub fn summarize(lines: &[LogLine]) -> LogStats {
    let mut by_level: HashMap<String, usize> = HashMap::new();
    let mut message_counts: HashMap<String, usize> = HashMap::new();
    let mut without = 0;

    for line in lines {
        match &line.level {
            Some(lvl) => *by_level.entry(lvl.clone()).or_insert(0) += 1,
            None => without += 1,
        }
        // Count repeated raw messages (useful for detecting log spam)
        *message_counts.entry(line.raw.clone()).or_insert(0) += 1;
    }

    // Top 5 most repeated messages
    let mut sorted: Vec<(String, usize)> = message_counts
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .collect();
    sorted.sort_by_key(|b| std::cmp::Reverse(b.1));
    sorted.truncate(5);

    LogStats {
        total_lines: lines.len(),
        by_level,
        lines_without_level: without,
        top_repeated: sorted,
    }
}

#[allow(dead_code)]
pub fn filter_by_level<'a>(lines: &'a [LogLine], level: &str) -> Vec<&'a LogLine> {
    let target = level.to_uppercase();
    lines
        .iter()
        .filter(|l| l.level.as_ref().map(|x| x.to_uppercase()) == Some(target.clone()))
        .collect()
}

#[allow(dead_code)]
pub fn errors_only(lines: &[LogLine]) -> Vec<&LogLine> {
    filter_by_level(lines, "ERROR")
}

/// Keep only lines whose line_number is within [start, end] inclusive.
#[allow(dead_code)]
pub fn line_range(lines: &[LogLine], start: usize, end: usize) -> Vec<&LogLine> {
    lines
        .iter()
        .filter(|l| l.line_number >= start && l.line_number <= end)
        .collect()
}

/// Keep only lines whose severity is at or above the given threshold.
#[allow(dead_code)]
pub fn at_least_severity(lines: &[LogLine], min: crate::parser::Severity) -> Vec<&LogLine> {
    lines
        .iter()
        .filter(|l| {
            l.level
                .as_deref()
                .and_then(crate::parser::Severity::from_str)
                .map(|s| s >= min)
                .unwrap_or(false)
        })
        .collect()
}

/// Extract all unique substrings from `raw` that match the given regex pattern.
#[allow(dead_code)]
pub fn extract_matches(lines: &[LogLine], pattern: &str) -> Result<Vec<String>, regex::Error> {
    let re = regex::Regex::new(pattern)?;
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for line in lines {
        for m in re.find_iter(&line.raw) {
            let s = m.as_str().to_string();
            if seen.insert(s.clone()) {
                out.push(s);
            }
        }
    }
    Ok(out)
}

/// Extract all unique IPv4 addresses found in log content.
#[allow(dead_code)]
pub fn extract_ipv4(lines: &[LogLine]) -> Vec<String> {
    let pattern = r"\b(?:\d{1,3}\.){3}\d{1,3}\b";
    extract_matches(lines, pattern).unwrap_or_default()
}

/// Extract HTTP status codes (3-digit numbers in 1xx-5xx range) with their occurrence counts.
#[allow(dead_code)]
pub fn http_status_counts(lines: &[LogLine]) -> std::collections::HashMap<u16, usize> {
    let re = regex::Regex::new(r"\b([1-5]\d{2})\b").unwrap();
    let mut counts = std::collections::HashMap::new();
    for line in lines {
        for cap in re.captures_iter(&line.raw) {
            if let Ok(code) = cap[1].parse::<u16>() {
                *counts.entry(code).or_insert(0) += 1;
            }
        }
    }
    counts
}

/// Extract UUID-like session/correlation IDs and count their occurrences.
#[allow(dead_code)]
pub fn session_id_counts(lines: &[LogLine]) -> std::collections::HashMap<String, usize> {
    let re = regex::Regex::new(
        r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b",
    )
    .unwrap();
    let mut counts = std::collections::HashMap::new();
    for line in lines {
        for m in re.find_iter(&line.raw) {
            *counts.entry(m.as_str().to_string()).or_insert(0) += 1;
        }
    }
    counts
}

/// Extract URLs from log content.
#[allow(dead_code)]
pub fn extract_urls(lines: &[LogLine]) -> Vec<String> {
    let re = regex::Regex::new(r"https?://[^\s\)\]\}]+").unwrap();
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for line in lines {
        for m in re.find_iter(&line.raw) {
            let s = m.as_str().to_string();
            if seen.insert(s.clone()) {
                out.push(s);
            }
        }
    }
    out
}

/// Extract Unix-style file paths from log content.
#[allow(dead_code)]
pub fn extract_paths(lines: &[LogLine]) -> Vec<String> {
    let re = regex::Regex::new(r"(?:^|\s)(/[A-Za-z0-9_./\-]+)").unwrap();
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for line in lines {
        for cap in re.captures_iter(&line.raw) {
            if let Some(m) = cap.get(1) {
                let s = m.as_str().to_string();
                if seen.insert(s.clone()) {
                    out.push(s);
                }
            }
        }
    }
    out
}

/// Group IPv4 addresses by /24 subnet prefix and count occurrences.
#[allow(dead_code)]
pub fn ip_subnet_counts(lines: &[LogLine]) -> std::collections::HashMap<String, usize> {
    let ips = extract_ipv4(lines);
    let mut counts = std::collections::HashMap::new();
    for ip in ips {
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() == 4 {
            let subnet = format!("{}.{}.{}.0/24", parts[0], parts[1], parts[2]);
            *counts.entry(subnet).or_insert(0) += 1;
        }
    }
    counts
}

/// Classify a line into a coarse error category based on keywords.
#[allow(dead_code)]
pub fn classify_error(line: &LogLine) -> Option<&'static str> {
    let raw = line.raw.to_lowercase();
    if raw.contains("timeout") || raw.contains("timed out") {
        Some("timeout")
    } else if raw.contains("connection refused") || raw.contains("refused") {
        Some("connection")
    } else if raw.contains("permission denied")
        || raw.contains("forbidden")
        || raw.contains("unauthorized")
    {
        Some("auth")
    } else if raw.contains("not found") || raw.contains("404") {
        Some("not_found")
    } else if raw.contains("out of memory") || raw.contains("oom") {
        Some("memory")
    } else if raw.contains("deadlock") || raw.contains("lock wait") {
        Some("concurrency")
    } else if raw.contains("syntax error") || raw.contains("parse error") {
        Some("syntax")
    } else if raw.contains("disk full") || raw.contains("no space") {
        Some("disk")
    } else {
        None
    }
}

/// Group lines by error category, returning counts per category.
#[allow(dead_code)]
pub fn error_category_counts(lines: &[LogLine]) -> std::collections::HashMap<&'static str, usize> {
    let mut counts = std::collections::HashMap::new();
    for line in lines {
        if let Some(cat) = classify_error(line) {
            *counts.entry(cat).or_insert(0) += 1;
        }
    }
    counts
}

/// Extract values for a named JSON field across all lines that parse as JSON.
#[allow(dead_code)]
pub fn extract_json_field(lines: &[LogLine], field: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in lines {
        if !crate::parser::is_json_line(&line.raw) {
            continue;
        }
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&line.raw) {
            if let Some(val) = v.get(field) {
                let s = match val {
                    serde_json::Value::String(s) => s.clone(),
                    other => other.to_string(),
                };
                out.push(s);
            }
        }
    }
    out
}

/// Count distinct values for a JSON field.
#[allow(dead_code)]
pub fn json_field_counts(
    lines: &[LogLine],
    field: &str,
) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
    for v in extract_json_field(lines, field) {
        *counts.entry(v).or_insert(0) += 1;
    }
    counts
}

/// Extract latency values in milliseconds from log content.
/// Matches patterns like "took 123ms", "elapsed=45ms", "duration: 1234 ms".
#[allow(dead_code)]
pub fn extract_latencies_ms(lines: &[LogLine]) -> Vec<u64> {
    let re = regex::Regex::new(r"(\d+)\s*ms\b").unwrap();
    let mut out = Vec::new();
    for line in lines {
        for cap in re.captures_iter(&line.raw) {
            if let Ok(v) = cap[1].parse::<u64>() {
                out.push(v);
            }
        }
    }
    out
}

/// Mean latency across all extracted samples.
#[allow(dead_code)]
pub fn mean_latency_ms(lines: &[LogLine]) -> f64 {
    let v = extract_latencies_ms(lines);
    if v.is_empty() {
        return 0.0;
    }
    v.iter().sum::<u64>() as f64 / v.len() as f64
}

/// For each IP, count how many lines reference it and what levels they appear at.
#[allow(dead_code)]
pub fn ip_activity(
    lines: &[LogLine],
) -> std::collections::HashMap<String, std::collections::HashMap<String, usize>> {
    let re = regex::Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();
    let mut out: std::collections::HashMap<String, std::collections::HashMap<String, usize>> =
        std::collections::HashMap::new();
    for line in lines {
        let level = line
            .level
            .clone()
            .unwrap_or_else(|| "UNKNOWN".to_string())
            .to_uppercase();
        for m in re.find_iter(&line.raw) {
            let ip_entry = out.entry(m.as_str().to_string()).or_default();
            *ip_entry.entry(level.clone()).or_insert(0) += 1;
        }
    }
    out
}

/// Find IPs that have any ERROR-level activity.
#[allow(dead_code)]
pub fn suspect_ips(lines: &[LogLine]) -> Vec<String> {
    let activity = ip_activity(lines);
    let mut suspects: Vec<String> = activity
        .into_iter()
        .filter(|(_, levels)| levels.contains_key("ERROR"))
        .map(|(ip, _)| ip)
        .collect();
    suspects.sort();
    suspects
}

fn ipv4_to_u32(ip: &str) -> Option<u32> {
    let parts: Vec<&str> = ip.split('.').collect();
    if parts.len() != 4 {
        return None;
    }
    let mut acc: u32 = 0;
    for p in parts {
        let n: u32 = p.parse().ok()?;
        if n > 255 {
            return None;
        }
        acc = (acc << 8) | n;
    }
    Some(acc)
}

/// Keep only lines containing an IPv4 inside the given CIDR (e.g. "10.0.0.0/8").
#[allow(dead_code)]
pub fn in_cidr<'a>(lines: &'a [LogLine], cidr: &str) -> Vec<&'a LogLine> {
    let parts: Vec<&str> = cidr.split('/').collect();
    if parts.len() != 2 {
        return Vec::new();
    }
    let base = match ipv4_to_u32(parts[0]) {
        Some(v) => v,
        None => return Vec::new(),
    };
    let bits: u32 = match parts[1].parse() {
        Ok(v) if v <= 32 => v,
        _ => return Vec::new(),
    };
    let mask: u32 = if bits == 0 { 0 } else { !0u32 << (32 - bits) };
    let target = base & mask;
    let re = regex::Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();
    lines
        .iter()
        .filter(|l| {
            re.find_iter(&l.raw)
                .filter_map(|m| ipv4_to_u32(m.as_str()))
                .any(|ip| (ip & mask) == target)
        })
        .collect()
}

/// Count lines whose raw content contains the given substring.
#[allow(dead_code)]
pub fn count_matching(lines: &[LogLine], needle: &str) -> usize {
    lines.iter().filter(|l| l.raw.contains(needle)).count()
}

/// Count lines whose raw content matches the given regex.
#[allow(dead_code)]
pub fn count_regex(lines: &[LogLine], pattern: &str) -> Result<usize, regex::Error> {
    let re = regex::Regex::new(pattern)?;
    Ok(lines.iter().filter(|l| re.is_match(&l.raw)).count())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::LogLine;

    fn sample_lines() -> Vec<LogLine> {
        vec![
            LogLine {
                line_number: 1,
                level: Some("INFO".into()),
                raw: "INFO starting".into(),
            },
            LogLine {
                line_number: 2,
                level: Some("ERROR".into()),
                raw: "ERROR oops".into(),
            },
            LogLine {
                line_number: 3,
                level: Some("INFO".into()),
                raw: "INFO running".into(),
            },
            LogLine {
                line_number: 4,
                level: None,
                raw: "plain line".into(),
            },
        ]
    }

    #[test]
    fn filters_by_level() {
        let out = apply(&sample_lines(), Some("INFO"), None).unwrap();
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn filters_by_pattern() {
        let out = apply(&sample_lines(), None, Some("oops")).unwrap();
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn filters_combined() {
        let out = apply(&sample_lines(), Some("INFO"), Some("running")).unwrap();
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn summary_counts_correctly() {
        let s = summarize(&sample_lines());
        assert_eq!(s.total_lines, 4);
        assert_eq!(s.by_level.get("INFO"), Some(&2));
        assert_eq!(s.by_level.get("ERROR"), Some(&1));
        assert_eq!(s.lines_without_level, 1);
        assert!(s.top_repeated.len() <= 5);
    }

    #[test]
    fn invalid_regex_errors() {
        let r = apply(&sample_lines(), None, Some("[invalid"));
        assert!(r.is_err());
    }

    #[test]
    fn filters_by_time_prefix() {
        let lines = vec![
            LogLine {
                line_number: 1,
                level: Some("INFO".into()),
                raw: "2026-04-16 10:00:01 INFO start".into(),
            },
            LogLine {
                line_number: 2,
                level: Some("ERROR".into()),
                raw: "2026-04-16 10:00:10 ERROR fail".into(),
            },
            LogLine {
                line_number: 3,
                level: Some("INFO".into()),
                raw: "2026-04-16 10:00:15 INFO ok".into(),
            },
        ];

        let out = super::filter_by_time_prefix(&lines, "2026-04-16 10:00:1");
        assert_eq!(out.len(), 2);
    }

    #[test]
    fn at_least_severity_filters_below_threshold() {
        let lines = sample_lines();
        let got = super::at_least_severity(&lines, crate::parser::Severity::Error);
        assert_eq!(got.len(), 1);
    }

    #[test]
    fn line_range_inclusive() {
        let lines = sample_lines();
        let got = super::line_range(&lines, 2, 3);
        assert_eq!(got.len(), 2);
        assert_eq!(got[0].line_number, 2);
        assert_eq!(got[1].line_number, 3);
    }
}

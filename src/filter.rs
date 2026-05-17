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

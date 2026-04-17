//! Filtering and summarizing log lines.

use crate::parser::LogLine;
use regex::Regex;
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FilterError {
    #[error("invalid regex pattern: {0}")]
    InvalidRegex(#[from] regex::Error),
}

/// Apply level and pattern filters. Both are optional.
/// If a filter is None, that criterion is not applied.
pub fn apply<'a>(
    lines: &'a [LogLine],
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
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.truncate(5);

    LogStats {
        total_lines: lines.len(),
        by_level,
        lines_without_level: without,
        top_repeated: sorted,
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::LogLine;

    fn sample_lines() -> Vec<LogLine> {
        vec![
            LogLine { line_number: 1, level: Some("INFO".into()), raw: "INFO starting".into() },
            LogLine { line_number: 2, level: Some("ERROR".into()), raw: "ERROR oops".into() },
            LogLine { line_number: 3, level: Some("INFO".into()), raw: "INFO running".into() },
            LogLine { line_number: 4, level: None, raw: "plain line".into() },
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
            LogLine { line_number: 1, level: Some("INFO".into()), raw: "2026-04-16 10:00:01 INFO start".into() },
            LogLine { line_number: 2, level: Some("ERROR".into()), raw: "2026-04-16 10:00:10 ERROR fail".into() },
            LogLine { line_number: 3, level: Some("INFO".into()), raw: "2026-04-16 10:00:15 INFO ok".into() },
        ];

        let out = super::filter_by_time_prefix(&lines, "2026-04-16 10:00:1");
        assert_eq!(out.len(), 2);
    }
}
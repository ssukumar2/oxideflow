//! Log file parsing. Tries to recognize common log formats.

use colored::Colorize;
use serde::Serialize;
use std::fmt;
use std::fs;
use std::path::Path;

/// A single parsed log line.
#[derive(Debug, Clone, Serialize)]
pub struct LogLine {
    /// Line number in the original file (1-indexed)
    pub line_number: usize,
    /// Detected log level, if any
    pub level: Option<String>,
    /// Raw log line text
    pub raw: String,
}

impl fmt::Display for LogLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level_str = match &self.level {
            Some(lvl) => match lvl.as_str() {
                "ERROR" => format!("{:>5}", lvl).red().bold().to_string(),
                "WARN" | "WARNING" => format!("{:>5}", "WARN").yellow().bold().to_string(),
                "INFO" => format!("{:>5}", lvl).green().to_string(),
                "DEBUG" => format!("{:>5}", lvl).blue().to_string(),
                "TRACE" => format!("{:>5}", lvl).dimmed().to_string(),
                _ => format!("{:>5}", lvl),
            },
            None => format!("{:>5}", "---"),
        };
        write!(f, "{:>6}  {}  {}", self.line_number, level_str, self.raw)
    }
}

/// Read a log file and return parsed lines.
pub fn read_file(path: &Path) -> std::io::Result<Vec<LogLine>> {
    let content = fs::read_to_string(path)?;
    Ok(parse_content(&content))
}

fn parse_content(content: &str) -> Vec<LogLine> {
    content
        .lines()
        .enumerate()
        .map(|(idx, raw)| LogLine {
            line_number: idx + 1,
            level: detect_level(raw),
            raw: raw.to_string(),
        })
        .collect()
}

/// Best-effort detection of common log levels.
fn detect_level(line: &str) -> Option<String> {
    const LEVELS: &[&str] = &["ERROR", "WARN", "WARNING", "INFO", "DEBUG", "TRACE"];
    let upper = line.to_uppercase();
    for &lvl in LEVELS {
        if upper.contains(lvl) {
            return Some(lvl.to_string());
        }
    }
    None
}

pub fn parse_line(raw: &str, line_number: usize) -> LogLine {
    let level = if raw.contains("ERROR") {
        Some("ERROR".to_string())
    } else if raw.contains("WARN") {
        Some("WARN".to_string())
    } else if raw.contains("INFO") {
        Some("INFO".to_string())
    } else if raw.contains("DEBUG") {
        Some("DEBUG".to_string())
    } else if raw.contains("TRACE") {
        Some("TRACE".to_string())
    } else {
        None
    };

    LogLine {
        line_number,
        level,
        raw: raw.to_string(),
    }
}

/// Log severity ordered from least to most severe.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[allow(dead_code)]
impl Severity {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TRACE" => Some(Self::Trace),
            "DEBUG" => Some(Self::Debug),
            "INFO" => Some(Self::Info),
            "WARN" | "WARNING" => Some(Self::Warn),
            "ERROR" => Some(Self::Error),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        }
    }
}

/// Returns true if the line's raw content is empty or whitespace-only.
#[allow(dead_code)]
pub fn is_blank(line: &LogLine) -> bool {
    line.raw.trim().is_empty()
}

/// Total bytes of raw content across all lines.
#[allow(dead_code)]
pub fn total_bytes(lines: &[LogLine]) -> usize {
    lines.iter().map(|l| l.raw.len()).sum()
}

/// Average line length in bytes (returns 0 for empty input).
#[allow(dead_code)]
pub fn average_line_length(lines: &[LogLine]) -> usize {
    if lines.is_empty() {
        return 0;
    }
    total_bytes(lines) / lines.len()
}

/// Return the first `n` lines (or all if fewer exist).
#[allow(dead_code)]
pub fn head_n(lines: &[LogLine], n: usize) -> Vec<&LogLine> {
    lines.iter().take(n).collect()
}

/// Return the last `n` lines (or all if fewer exist).
#[allow(dead_code)]
pub fn tail_n(lines: &[LogLine], n: usize) -> Vec<&LogLine> {
    let len = lines.len();
    let start = len.saturating_sub(n);
    lines[start..].iter().collect()
}

/// Identify lines that look like stack trace frames (indented "at" patterns).
#[allow(dead_code)]
pub fn is_stack_frame(line: &LogLine) -> bool {
    let trimmed = line.raw.trim_start();
    trimmed.starts_with("at ")
        || trimmed.starts_with("Caused by:")
        || trimmed.starts_with("... ")
        || trimmed.starts_with("File \"")
}

/// Group consecutive stack frames under the preceding error line.
#[allow(dead_code)]
pub fn group_stack_traces(lines: &[LogLine]) -> Vec<Vec<&LogLine>> {
    let mut groups: Vec<Vec<&LogLine>> = Vec::new();
    let mut current: Vec<&LogLine> = Vec::new();
    for l in lines {
        if is_stack_frame(l) {
            current.push(l);
        } else {
            if !current.is_empty() {
                groups.push(std::mem::take(&mut current));
            }
            current.push(l);
        }
    }
    if !current.is_empty() {
        groups.push(current);
    }
    groups
}

/// Read a file line-by-line, calling `callback` for each parsed LogLine.
/// Memory-efficient for files too large to load entirely.
#[allow(dead_code)]
pub fn read_streaming<F>(path: &std::path::Path, mut callback: F) -> std::io::Result<usize>
where
    F: FnMut(LogLine),
{
    use std::io::BufRead;
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut count = 0usize;
    for (i, line_res) in reader.lines().enumerate() {
        let raw = line_res?;
        let parsed = parse_line(&raw, i + 1);
        callback(parsed);
        count += 1;
    }
    Ok(count)
}

/// Normalize all level variants to canonical uppercase forms.
/// WARNING/WARN → WARN, ERR/ERROR → ERROR, etc.
#[allow(dead_code)]
pub fn normalize_levels(lines: &[LogLine]) -> Vec<LogLine> {
    lines
        .iter()
        .map(|l| {
            let normalized = l
                .level
                .as_deref()
                .map(|lvl| match lvl.to_uppercase().as_str() {
                    "WARNING" | "WARN" => "WARN".to_string(),
                    "ERR" | "ERROR" | "FATAL" | "CRITICAL" => "ERROR".to_string(),
                    "DBG" | "DEBUG" => "DEBUG".to_string(),
                    "INF" | "INFO" => "INFO".to_string(),
                    "TRC" | "TRACE" | "VERBOSE" => "TRACE".to_string(),
                    other => other.to_string(),
                });
            LogLine {
                line_number: l.line_number,
                level: normalized,
                raw: l.raw.clone(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_error_level() {
        assert_eq!(
            detect_level("2026-04-16 ERROR something broke"),
            Some("ERROR".into())
        );
    }

    #[test]
    fn detects_info_level() {
        assert_eq!(
            detect_level("2026-04-16 INFO server started"),
            Some("INFO".into())
        );
    }

    #[test]
    fn detects_no_level_when_absent() {
        assert_eq!(detect_level("just a plain line"), None);
    }

    #[test]
    fn parses_multiline_content() {
        let content = "INFO starting\nERROR oops\nINFO done";
        let lines = parse_content(content);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0].line_number, 1);
        assert_eq!(lines[1].level, Some("ERROR".into()));
    }

    #[test]
    fn severity_parses_case_insensitive() {
        assert_eq!(
            super::Severity::from_str("error"),
            Some(super::Severity::Error)
        );
        assert_eq!(
            super::Severity::from_str("WARNING"),
            Some(super::Severity::Warn)
        );
        assert_eq!(super::Severity::from_str("bogus"), None);
    }

    #[test]
    fn severity_ordering() {
        assert!(super::Severity::Error > super::Severity::Warn);
        assert!(super::Severity::Trace < super::Severity::Info);
    }

    #[test]
    fn severity_as_str_roundtrip() {
        let s = super::Severity::Info;
        assert_eq!(s.as_str(), "INFO");
    }
}

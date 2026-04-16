//! Log file parsing. Tries to recognize common log formats.

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
        write!(f, "{:>6}  {}", self.line_number, self.raw)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_error_level() {
        assert_eq!(detect_level("2026-04-16 ERROR something broke"), Some("ERROR".into()));
    }

    #[test]
    fn detects_info_level() {
        assert_eq!(detect_level("2026-04-16 INFO server started"), Some("INFO".into()));
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
}
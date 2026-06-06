//! Tag log lines with derived metadata for downstream filtering.

use crate::parser::LogLine;
use std::collections::HashSet;

#[allow(dead_code)]
pub struct TaggedLine<'a> {
    pub line: &'a LogLine,
    pub tags: HashSet<&'static str>,
}

#[allow(dead_code)]
pub fn auto_tag(lines: &[LogLine]) -> Vec<TaggedLine<'_>> {
    lines
        .iter()
        .map(|l| {
            let mut tags: HashSet<&'static str> = HashSet::new();
            let raw_lower = l.raw.to_lowercase();
            if raw_lower.contains("http") {
                tags.insert("http");
            }
            if raw_lower.contains("sql") || raw_lower.contains("query") {
                tags.insert("database");
            }
            if raw_lower.contains("login") || raw_lower.contains("auth") {
                tags.insert("auth");
            }
            if raw_lower.contains("cache") {
                tags.insert("cache");
            }
            if raw_lower.contains("retry") || raw_lower.contains("retrying") {
                tags.insert("retry");
            }
            if l.level
                .as_deref()
                .map(|s| s.eq_ignore_ascii_case("ERROR"))
                .unwrap_or(false)
            {
                tags.insert("error");
            }
            TaggedLine { line: l, tags }
        })
        .collect()
}

#[allow(dead_code)]
pub fn lines_with_tag<'a>(tagged: &'a [TaggedLine<'a>], tag: &str) -> Vec<&'a LogLine> {
    tagged
        .iter()
        .filter(|t| t.tags.contains(tag))
        .map(|t| t.line)
        .collect()
}

//! Trace transactions by correlation ID across log lines.

use crate::parser::LogLine;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Trace<'a> {
    pub id: String,
    pub lines: Vec<&'a LogLine>,
    pub span_lines: usize,
}

/// Group lines sharing the same UUID-like correlation ID.
/// Returns traces sorted by line count descending.
#[allow(dead_code)]
pub fn trace_by_uuid(lines: &[LogLine]) -> Vec<Trace<'_>> {
    let re = regex::Regex::new(
        r"\b[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}\b",
    )
    .unwrap();
    let mut groups: HashMap<String, Vec<&LogLine>> = HashMap::new();
    for line in lines {
        if let Some(m) = re.find(&line.raw) {
            groups.entry(m.as_str().to_string()).or_default().push(line);
        }
    }
    let mut traces: Vec<Trace> = groups
        .into_iter()
        .map(|(id, ls)| {
            let first = ls.first().map(|l| l.line_number).unwrap_or(0);
            let last = ls.last().map(|l| l.line_number).unwrap_or(0);
            Trace {
                id,
                span_lines: last.saturating_sub(first),
                lines: ls,
            }
        })
        .collect();
    traces.sort_by_key(|t| std::cmp::Reverse(t.lines.len()));
    traces
}

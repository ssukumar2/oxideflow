//! Pattern search with surrounding context lines.

use crate::parser::LogLine;

/// A match with its surrounding context lines.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ContextMatch<'a> {
    pub before: Vec<&'a LogLine>,
    pub matched: &'a LogLine,
    pub after: Vec<&'a LogLine>,
}

/// Search for `pattern` (regex) and include `context` lines before and after each hit.
#[allow(dead_code)]
pub fn search_with_context<'a>(
    lines: &'a [LogLine],
    pattern: &str,
    context: usize,
) -> Result<Vec<ContextMatch<'a>>, regex::Error> {
    let re = regex::Regex::new(pattern)?;
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if re.is_match(&line.raw) {
            let start = i.saturating_sub(context);
            let end = (i + context + 1).min(lines.len());
            let before: Vec<&LogLine> = lines[start..i].iter().collect();
            let after: Vec<&LogLine> = lines[i + 1..end].iter().collect();
            out.push(ContextMatch {
                before,
                matched: line,
                after,
            });
        }
    }
    Ok(out)
}

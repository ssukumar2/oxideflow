//! Redact sensitive patterns (emails, tokens, IPs) from log content.

use crate::parser::LogLine;

/// Replace email addresses in raw content with `[REDACTED_EMAIL]`.
#[allow(dead_code)]
pub fn redact_emails(lines: &[LogLine]) -> Vec<LogLine> {
    let re = regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}\b").unwrap();
    lines
        .iter()
        .map(|l| LogLine {
            line_number: l.line_number,
            level: l.level.clone(),
            raw: re.replace_all(&l.raw, "[REDACTED_EMAIL]").to_string(),
        })
        .collect()
}

/// Replace bearer tokens and API keys (long alphanumeric strings) with `[REDACTED_TOKEN]`.
#[allow(dead_code)]
pub fn redact_tokens(lines: &[LogLine]) -> Vec<LogLine> {
    let re = regex::Regex::new(r"\b[A-Za-z0-9_\-]{32,}\b").unwrap();
    lines
        .iter()
        .map(|l| LogLine {
            line_number: l.line_number,
            level: l.level.clone(),
            raw: re.replace_all(&l.raw, "[REDACTED_TOKEN]").to_string(),
        })
        .collect()
}

/// Apply all redactions in sequence.
#[allow(dead_code)]
pub fn redact_all(lines: &[LogLine]) -> Vec<LogLine> {
    let stage1 = redact_emails(lines);
    redact_tokens(&stage1)
}

/// Check if every line in the redacted set is indistinguishable from at least
/// `k-1` other lines (k-anonymity property).
#[allow(dead_code)]
pub fn satisfies_k_anonymity(lines: &[LogLine], k: usize) -> bool {
    let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for line in lines {
        *counts.entry(line.raw.as_str()).or_insert(0) += 1;
    }
    counts.values().all(|&c| c >= k)
}

/// Return the smallest equivalence class size in the redacted log.
#[allow(dead_code)]
pub fn min_equivalence_class(lines: &[LogLine]) -> usize {
    let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for line in lines {
        *counts.entry(line.raw.as_str()).or_insert(0) += 1;
    }
    counts.values().copied().min().unwrap_or(0)
}

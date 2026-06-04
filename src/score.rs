//! Score individual log lines by importance for triage.

use crate::parser::LogLine;

/// Score a line 0..100 based on level, keywords, and presence of error signals.
#[allow(dead_code)]
pub fn line_importance(line: &LogLine) -> u32 {
    let mut score: u32 = 0;
    if let Some(lvl) = line.level.as_deref() {
        score += match lvl.to_uppercase().as_str() {
            "ERROR" | "FATAL" | "CRITICAL" => 60,
            "WARN" | "WARNING" => 25,
            "INFO" => 5,
            _ => 0,
        };
    }
    let raw = line.raw.to_lowercase();
    let keywords = [
        ("panic", 30),
        ("fatal", 30),
        ("exception", 20),
        ("timeout", 15),
        ("refused", 15),
        ("denied", 10),
        ("retry", 5),
    ];
    for (kw, weight) in keywords {
        if raw.contains(kw) {
            score = score.saturating_add(weight);
        }
    }
    score.min(100)
}

/// Return the top N most-important lines.
#[allow(dead_code)]
pub fn top_important(lines: &[LogLine], n: usize) -> Vec<&LogLine> {
    let mut scored: Vec<(u32, &LogLine)> = lines.iter().map(|l| (line_importance(l), l)).collect();
    scored.sort_by_key(|x| std::cmp::Reverse(x.0));
    scored.into_iter().take(n).map(|(_, l)| l).collect()
}

//! Rule-based alerting on log content.

use crate::parser::LogLine;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Rule {
    /// Trigger when ERROR rate exceeds the given percentage.
    ErrorRateAbove(f64),
    /// Trigger when a specific pattern appears more than N times.
    PatternCountAbove { pattern: String, threshold: usize },
    /// Trigger when total line count exceeds N.
    TotalLinesAbove(usize),
    /// Trigger when any line contains any of the given substrings.
    AnySubstring(Vec<String>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Alert {
    pub rule: String,
    pub triggered: bool,
    pub detail: String,
}

#[allow(dead_code)]
pub fn evaluate(rules: &[Rule], lines: &[LogLine]) -> Vec<Alert> {
    rules
        .iter()
        .map(|rule| match rule {
            Rule::ErrorRateAbove(threshold) => {
                let rate = crate::stats::error_rate(lines);
                Alert {
                    rule: format!("error_rate > {:.2}%", threshold),
                    triggered: rate > *threshold,
                    detail: format!("actual: {:.2}%", rate),
                }
            }
            Rule::PatternCountAbove { pattern, threshold } => {
                let count = match regex::Regex::new(pattern) {
                    Ok(re) => lines.iter().filter(|l| re.is_match(&l.raw)).count(),
                    Err(_) => 0,
                };
                Alert {
                    rule: format!("count('{}') > {}", pattern, threshold),
                    triggered: count > *threshold,
                    detail: format!("actual: {}", count),
                }
            }
            Rule::TotalLinesAbove(threshold) => Alert {
                rule: format!("total_lines > {}", threshold),
                triggered: lines.len() > *threshold,
                detail: format!("actual: {}", lines.len()),
            },
            Rule::AnySubstring(needles) => {
                let hit = lines
                    .iter()
                    .find(|l| needles.iter().any(|n| l.raw.contains(n.as_str())));
                Alert {
                    rule: format!("any of {:?}", needles),
                    triggered: hit.is_some(),
                    detail: hit
                        .map(|l| format!("line {}", l.line_number))
                        .unwrap_or_else(|| "none".to_string()),
                }
            }
        })
        .collect()
}

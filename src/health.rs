//! Compute a single health score from log content.

use crate::parser::LogLine;

#[allow(dead_code)]
pub struct HealthScore {
    pub score: f64,
    pub error_rate: f64,
    pub warn_rate: f64,
    pub total_lines: usize,
    pub verdict: &'static str,
}

/// Score from 0.0 (worst) to 100.0 (perfect).
/// Subtracts 5 points per 1% error rate and 1 point per 1% warn rate.
#[allow(dead_code)]
pub fn compute(lines: &[LogLine]) -> HealthScore {
    let error_rate = crate::stats::error_rate(lines);
    let warn_rate = {
        if lines.is_empty() {
            0.0
        } else {
            let w = lines
                .iter()
                .filter(|l| {
                    l.level
                        .as_deref()
                        .map(|s| {
                            s.eq_ignore_ascii_case("WARN") || s.eq_ignore_ascii_case("WARNING")
                        })
                        .unwrap_or(false)
                })
                .count();
            (w as f64 / lines.len() as f64) * 100.0
        }
    };
    let raw = 100.0 - (error_rate * 5.0) - (warn_rate * 1.0);
    let score = raw.clamp(0.0, 100.0);
    let verdict = match score {
        x if x >= 90.0 => "healthy",
        x if x >= 70.0 => "stable",
        x if x >= 40.0 => "degraded",
        _ => "critical",
    };
    HealthScore {
        score,
        error_rate,
        warn_rate,
        total_lines: lines.len(),
        verdict,
    }
}

/// Quick yes/no check whether the log qualifies as "healthy" (>= 90 score).
#[allow(dead_code)]
pub fn is_healthy(lines: &[LogLine]) -> bool {
    compute(lines).score >= 90.0
}

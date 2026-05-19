//! Aggregate report combining stats, dedup, and anomaly outputs.

use crate::parser::LogLine;

#[allow(dead_code)]
pub struct Report {
    pub total_lines: usize,
    pub total_bytes: usize,
    pub error_rate: f64,
    pub level_breakdown: Vec<(String, f64)>,
    pub throughput_per_sec: f64,
    pub top_errors: Vec<(String, usize)>,
}

#[allow(dead_code)]
pub fn build_report(lines: &[LogLine]) -> Report {
    Report {
        total_lines: lines.len(),
        total_bytes: crate::parser::total_bytes(lines),
        error_rate: crate::stats::error_rate(lines),
        level_breakdown: crate::stats::level_percentages(lines),
        throughput_per_sec: crate::stats::throughput_per_sec(lines),
        top_errors: crate::dedup::top_errors(lines, 5),
    }
}

#[allow(dead_code)]
pub fn render_text(r: &Report) -> String {
    let mut out = String::new();
    out.push_str(&format!("Lines: {}\n", r.total_lines));
    out.push_str(&format!("Bytes: {}\n", r.total_bytes));
    out.push_str(&format!("Error rate: {:.2}%\n", r.error_rate));
    out.push_str(&format!(
        "Throughput: {:.2} lines/sec\n",
        r.throughput_per_sec
    ));
    out.push_str("Levels:\n");
    for (level, pct) in &r.level_breakdown {
        out.push_str(&format!("  {:<8} {:.2}%\n", level, pct));
    }
    if !r.top_errors.is_empty() {
        out.push_str("Top errors:\n");
        for (msg, count) in &r.top_errors {
            out.push_str(&format!("  [{}] {}\n", count, msg));
        }
    }
    out
}

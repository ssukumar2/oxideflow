//! End-to-end report combining health, alerts, top errors, and traces.

use crate::parser::LogLine;

#[allow(dead_code)]
pub struct FullReport {
    pub health: crate::health::HealthScore,
    pub summary: crate::summary::Report,
    pub alerts: Vec<crate::alert::Alert>,
    pub top_important_count: usize,
    pub trace_count: usize,
    pub uniqueness: f64,
}

#[allow(dead_code)]
pub fn build(lines: &[LogLine], alert_rules: &[crate::alert::Rule]) -> FullReport {
    FullReport {
        health: crate::health::compute(lines),
        summary: crate::summary::build_report(lines),
        alerts: crate::alert::evaluate(alert_rules, lines),
        top_important_count: crate::score::top_important(lines, 10).len(),
        trace_count: crate::correlate::trace_by_uuid(lines).len(),
        uniqueness: crate::compress::uniqueness_ratio(lines),
    }
}

#[allow(dead_code)]
pub fn render(r: &FullReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "=== oxideflow report ===\nHealth: {:.1}/100 ({})\n",
        r.health.score, r.health.verdict
    ));
    out.push_str(&format!(
        "Lines: {} | Bytes: {} | Uniqueness: {:.2}%\n",
        r.summary.total_lines,
        r.summary.total_bytes,
        r.uniqueness * 100.0
    ));
    out.push_str(&format!(
        "Error rate: {:.2}% | Throughput: {:.2} lines/sec\n",
        r.summary.error_rate, r.summary.throughput_per_sec
    ));
    out.push_str(&format!(
        "Traces: {} | High-importance lines: {}\n",
        r.trace_count, r.top_important_count
    ));
    if !r.alerts.is_empty() {
        out.push_str("\nAlerts:\n");
        for a in &r.alerts {
            out.push_str(&format!(
                "  [{}] {} -- {}\n",
                if a.triggered { "FIRED" } else { "ok" },
                a.rule,
                a.detail
            ));
        }
    }
    out
}

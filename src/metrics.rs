//! Export log statistics in Prometheus text format.

use crate::parser::LogLine;

#[allow(dead_code)]
pub fn to_prometheus(lines: &[LogLine]) -> String {
    let mut out = String::new();
    out.push_str("# HELP oxideflow_lines_total Total number of log lines\n");
    out.push_str("# TYPE oxideflow_lines_total counter\n");
    out.push_str(&format!("oxideflow_lines_total {}\n", lines.len()));

    out.push_str("# HELP oxideflow_bytes_total Total bytes of log content\n");
    out.push_str("# TYPE oxideflow_bytes_total counter\n");
    out.push_str(&format!(
        "oxideflow_bytes_total {}\n",
        crate::parser::total_bytes(lines)
    ));

    out.push_str("# HELP oxideflow_lines_by_level Lines by severity level\n");
    out.push_str("# TYPE oxideflow_lines_by_level gauge\n");
    let counts = crate::stats::count_levels(lines);
    for (level, count) in counts {
        out.push_str(&format!(
            "oxideflow_lines_by_level{{level=\"{}\"}} {}\n",
            level, count
        ));
    }

    out.push_str("# HELP oxideflow_error_rate Percentage of lines at ERROR level\n");
    out.push_str("# TYPE oxideflow_error_rate gauge\n");
    out.push_str(&format!(
        "oxideflow_error_rate {:.4}\n",
        crate::stats::error_rate(lines)
    ));

    out
}

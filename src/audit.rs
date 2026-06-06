//! Record analysis runs for later inspection and reproducibility.

use crate::parser::LogLine;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub timestamp_unix: u64,
    pub input_fingerprint: u64,
    pub line_count: usize,
    pub byte_count: usize,
    pub operation: String,
    pub result_summary: String,
}

#[allow(dead_code)]
pub fn record(lines: &[LogLine], operation: &str, result: &str) -> AuditEntry {
    let timestamp_unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    AuditEntry {
        timestamp_unix,
        input_fingerprint: crate::parser::fingerprint(lines),
        line_count: lines.len(),
        byte_count: crate::parser::total_bytes(lines),
        operation: operation.to_string(),
        result_summary: result.to_string(),
    }
}

#[allow(dead_code)]
pub fn render_log(entries: &[AuditEntry]) -> String {
    let mut out = String::from("timestamp\tfingerprint\tlines\tbytes\toperation\tresult\n");
    for e in entries {
        out.push_str(&format!(
            "{}\t{:016x}\t{}\t{}\t{}\t{}\n",
            e.timestamp_unix,
            e.input_fingerprint,
            e.line_count,
            e.byte_count,
            e.operation,
            e.result_summary
        ));
    }
    out
}

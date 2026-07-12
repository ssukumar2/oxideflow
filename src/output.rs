use crate::parser::LogLine;
use colored::*;

pub fn print_colored(entry: &LogLine) {
    let level = entry.level.as_deref().unwrap_or("UNKNOWN");
    let colored_level = match level {
        "ERROR" => level.red().bold(),
        "WARN" => level.yellow().bold(),
        "INFO" => level.green(),
        "DEBUG" => level.blue(),
        "TRACE" => level.dimmed(),
        _ => level.normal(),
    };
    println!("[{}] {}", colored_level, entry.raw);
}

pub fn print_json(entry: &LogLine) {
    println!(
        "{{\"line\":{},\"level\":\"{}\",\"text\":\"{}\"}}",
        entry.line_number,
        entry.level.as_deref().unwrap_or("UNKNOWN"),
        entry.raw.replace('\\', "\\\\").replace('"', "\\\"")
    );
}

pub fn print_plain(entry: &LogLine) {
    println!("{}", entry.raw);
}

pub fn to_json(lines: &[crate::parser::LogLine]) -> String {
    let arr: Vec<_> = lines
        .iter()
        .map(|l| {
            serde_json::json!({
                "line_number": l.line_number,
                "level": l.level,
                "raw": l.raw,
            })
        })
        .collect();
    serde_json::to_string_pretty(&arr).unwrap_or_else(|_| "[]".to_string())
}

pub fn print_colored_all(lines: &[crate::parser::LogLine]) {
    use colored::Colorize;
    for line in lines {
        let level_str = line.level.as_deref().unwrap_or("UNKNOWN").to_uppercase();
        let colored = match level_str.as_str() {
            "ERROR" => level_str.red().bold(),
            "WARN" | "WARNING" => level_str.yellow().bold(),
            "INFO" => level_str.green(),
            "DEBUG" => level_str.blue(),
            "TRACE" => level_str.magenta(),
            _ => level_str.normal(),
        };
        println!("{:>6} [{}] {}", line.line_number, colored, line.raw);
    }
}

/// Format a brief one-line summary string for the given counts.
#[allow(dead_code)]
pub fn summary_line(total: usize, errors: usize, warns: usize) -> String {
    format!("total={} errors={} warns={}", total, errors, warns)
}

/// Serialize lines as CSV: line_number,level,raw (raw is double-quote escaped).
#[allow(dead_code)]
pub fn to_csv(lines: &[crate::parser::LogLine]) -> String {
    let mut out = String::from("line_number,level,raw\n");
    for line in lines {
        let level = line.level.as_deref().unwrap_or("");
        let escaped_raw = line.raw.replace('"', "\"\"");
        out.push_str(&format!(
            "{},{},\"{}\"\n",
            line.line_number, level, escaped_raw
        ));
    }
    out
}

/// Serialize lines as newline-delimited JSON (one object per line).
#[allow(dead_code)]
pub fn to_ndjson(lines: &[crate::parser::LogLine]) -> String {
    let mut out = String::new();
    for line in lines {
        let obj = serde_json::json!({
            "line_number": line.line_number,
            "level": line.level,
            "raw": line.raw,
        });
        out.push_str(&serde_json::to_string(&obj).unwrap_or_default());
        out.push('\n');
    }
    out
}

/// Render a markdown report from a summary::Report.
#[allow(dead_code)]
pub fn report_to_markdown(r: &crate::summary::Report) -> String {
    let mut out = String::new();
    out.push_str("# Log Analysis Report\n\n");
    out.push_str(&format!("- **Total lines**: {}\n", r.total_lines));
    out.push_str(&format!("- **Total bytes**: {}\n", r.total_bytes));
    out.push_str(&format!("- **Error rate**: {:.2}%\n", r.error_rate));
    out.push_str(&format!(
        "- **Throughput**: {:.2} lines/sec\n\n",
        r.throughput_per_sec
    ));
    out.push_str("## Level distribution\n\n");
    for (level, pct) in &r.level_breakdown {
        out.push_str(&format!("- `{}`: {:.2}%\n", level, pct));
    }
    if !r.top_errors.is_empty() {
        out.push_str("\n## Top errors\n\n");
        for (msg, count) in &r.top_errors {
            out.push_str(&format!("- ({}) `{}`\n", count, msg));
        }
    }
    out
}

/// Serialize lines as YAML (no external dep, simple by-hand format).
#[allow(dead_code)]
pub fn to_yaml(lines: &[crate::parser::LogLine]) -> String {
    let mut out = String::from("lines:\n");
    for l in lines {
        out.push_str(&format!("  - line_number: {}\n", l.line_number));
        let level = l.level.as_deref().unwrap_or("null");
        out.push_str(&format!("    level: {}\n", level));
        let escaped = l.raw.replace('"', "\\\"").replace('\n', "\\n");
        out.push_str(&format!("    raw: \"{}\"\n", escaped));
    }
    out
}

/// Render log lines as a standalone HTML table.
#[allow(dead_code)]
pub fn to_html(lines: &[crate::parser::LogLine]) -> String {
    let mut out = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><style>\
        body{font-family:monospace;background:#1e1e1e;color:#ddd;}\
        table{border-collapse:collapse;width:100%;}\
        th,td{padding:4px 8px;border-bottom:1px solid #333;text-align:left;}\
        .ERROR{color:#f55;}.WARN{color:#fb3;}.INFO{color:#5cf;}.DEBUG{color:#888;}\
        </style></head><body><table><tr><th>#</th><th>Level</th><th>Message</th></tr>",
    );
    for line in lines {
        let level = line.level.as_deref().unwrap_or("");
        let escaped = line
            .raw
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;");
        out.push_str(&format!(
            "<tr><td>{}</td><td class=\"{}\">{}</td><td>{}</td></tr>",
            line.line_number, level, level, escaped
        ));
    }
    out.push_str("</table></body></html>");
    out
}

/// Serialize lines as tab-separated values.
#[allow(dead_code)]
pub fn to_tsv(lines: &[crate::parser::LogLine]) -> String {
    let mut out = String::from("line_number\tlevel\traw\n");
    for line in lines {
        let level = line.level.as_deref().unwrap_or("");
        let raw = line.raw.replace(['\t', '\n'], " ");
        out.push_str(&format!("{}\t{}\t{}\n", line.line_number, level, raw));
    }
    out
}

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

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

pub fn print_plain(entry: &LogLine) {
    println!("{}", entry.raw);
}

use crate::parser::LogEntry;
use std::collections::HashMap;
use std::cmp::Reverse;

pub struct Stats {
    pub total: usize,
    pub by_level: HashMap<String, usize>,
    pub top_messages: Vec<(String, usize)>,
}

pub fn compute(entries: &[LogEntry]) -> Stats {
    let total = entries.len();
    let mut by_level: HashMap<String, usize> = HashMap::new();
    let mut msg_counts: HashMap<String, usize> = HashMap::new();

    for entry in entries {
        *by_level.entry(entry.level.clone()).or_insert(0) += 1;
        *msg_counts.entry(entry.message.clone()).or_insert(0) += 1;
    }

    let mut top: Vec<(String, usize)> = msg_counts.into_iter().collect();
    top.sort_by_key(|&(_, count)| Reverse(count));
    top.truncate(10);

    Stats {
        total,
        by_level,
        top_messages: top,
    }
}

pub fn print_stats(stats: &Stats) {
    println!("Total entries: {}", stats.total);
    println!("\nBy level:");
    let mut levels: Vec<_> = stats.by_level.iter().collect();
    levels.sort_by_key(|&(_, count)| Reverse(*count));
    for (level, count) in levels {
        println!("  {:<8} {}", level, count);
    }
    if !stats.top_messages.is_empty() {
        println!("\nTop repeated messages:");
        for (msg, count) in &stats.top_messages {
            println!("  [{:>4}] {}", count, msg);
        }
    }
}
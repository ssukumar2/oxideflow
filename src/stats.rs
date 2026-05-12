use crate::parser::LogLine;
use std::collections::HashMap;

pub fn count_levels(lines: &[LogLine]) -> HashMap<String, usize> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for line in lines {
        if let Some(level) = &line.level {
            *counts.entry(level.to_uppercase()).or_insert(0) += 1;
        } else {
            *counts.entry("UNKNOWN".to_string()).or_insert(0) += 1;
        }
    }
    counts
}

pub fn total_lines(lines: &[LogLine]) -> usize {
    lines.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::LogLine;

    fn mk(n: usize, level: Option<&str>) -> LogLine {
        LogLine {
            line_number: n,
            level: level.map(|s| s.to_string()),
            raw: format!("line {}", n),
        }
    }

    #[test]
    fn counts_by_level() {
        let lines = vec![
            mk(1, Some("ERROR")),
            mk(2, Some("error")),
            mk(3, Some("INFO")),
            mk(4, None),
        ];
        let c = count_levels(&lines);
        assert_eq!(c.get("ERROR"), Some(&2));
        assert_eq!(c.get("INFO"), Some(&1));
        assert_eq!(c.get("UNKNOWN"), Some(&1));
    }

    #[test]
    fn total_counts_all() {
        let lines = vec![mk(1, None), mk(2, Some("INFO"))];
        assert_eq!(total_lines(&lines), 2);
    }
}

use crate::parser::LogLine;
use regex::Regex;

pub struct TimeFilter {
    since: Option<String>,
    until: Option<String>,
}

impl TimeFilter {
    pub fn new(since: Option<String>, until: Option<String>) -> Self {
        Self { since, until }
    }

    pub fn apply<'a>(&self, entries: &'a [LogLine]) -> Vec<&'a LogLine> {
        let ts_re = Regex::new(r"\d{4}-\d{2}-\d{2}[T ]\d{2}:\d{2}:\d{2}").unwrap();

        entries
            .iter()
            .filter(|entry| {
                if let Some(caps) = ts_re.find(&entry.raw) {
                    let ts = caps.as_str();
                    if let Some(ref since) = self.since {
                        if ts < since.as_str() {
                            return false;
                        }
                    }
                    if let Some(ref until) = self.until {
                        if ts > until.as_str() {
                            return false;
                        }
                    }
                    true
                } else {
                    true // keep lines without timestamps
                }
            })
            .collect()
    }
}

/// Parse a HH:MM:SS substring at the start of `raw` into seconds since midnight.
#[allow(dead_code)]
pub fn parse_time_to_seconds(raw: &str) -> Option<u64> {
    let re = regex::Regex::new(r"(\d{2}):(\d{2}):(\d{2})").unwrap();
    let caps = re.captures(raw)?;
    let h: u64 = caps[1].parse().ok()?;
    let m: u64 = caps[2].parse().ok()?;
    let s: u64 = caps[3].parse().ok()?;
    if h > 23 || m > 59 || s > 59 {
        return None;
    }
    Some(h * 3600 + m * 60 + s)
}

/// Compute the time gap in seconds between consecutive log lines.
#[allow(dead_code)]
pub fn time_gaps(lines: &[crate::parser::LogLine]) -> Vec<i64> {
    let mut times: Vec<i64> = Vec::new();
    for l in lines {
        if let Some(t) = parse_time_to_seconds(&l.raw) {
            times.push(t as i64);
        }
    }
    times.windows(2).map(|w| w[1] - w[0]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::LogLine;

    fn make_line(raw: &str, num: usize) -> LogLine {
        LogLine {
            line_number: num,
            level: Some("INFO".to_string()),
            raw: raw.to_string(),
        }
    }

    #[test]
    fn test_since_filter() {
        let lines = vec![
            make_line("2024-01-01T10:00:00 first", 1),
            make_line("2024-01-02T10:00:00 second", 2),
            make_line("2024-01-03T10:00:00 third", 3),
        ];
        let f = TimeFilter::new(Some("2024-01-02".to_string()), None);
        let result = f.apply(&lines);
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_until_filter() {
        let lines = vec![
            make_line("2024-01-01T10:00:00 first", 1),
            make_line("2024-01-02T10:00:00 second", 2),
            make_line("2024-01-03T10:00:00 third", 3),
        ];
        let f = TimeFilter::new(None, Some("2024-01-02".to_string()));
        let result = f.apply(&lines);
        assert_eq!(result.len(), 1);
    }
}

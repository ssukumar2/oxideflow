//! Verify integrity of log content (gaps, ordering, malformed lines).

use crate::parser::LogLine;

#[allow(dead_code)]
pub struct IntegrityReport {
    pub line_number_gaps: Vec<(usize, usize)>,
    pub out_of_order_timestamps: usize,
    pub blank_lines: usize,
    pub no_level_lines: usize,
}

#[allow(dead_code)]
pub fn verify(lines: &[LogLine]) -> IntegrityReport {
    let mut gaps: Vec<(usize, usize)> = Vec::new();
    let mut prev_line: Option<usize> = None;
    for line in lines {
        if let Some(p) = prev_line {
            if line.line_number > p + 1 {
                gaps.push((p, line.line_number));
            }
        }
        prev_line = Some(line.line_number);
    }

    let re = regex::Regex::new(r"(\d{2}):(\d{2}):(\d{2})").unwrap();
    let mut prev_t: Option<u32> = None;
    let mut out_of_order = 0usize;
    for line in lines {
        if let Some(cap) = re.captures(&line.raw) {
            let h: u32 = cap[1].parse().unwrap_or(0);
            let m: u32 = cap[2].parse().unwrap_or(0);
            let s: u32 = cap[3].parse().unwrap_or(0);
            let t = h * 3600 + m * 60 + s;
            if let Some(prev) = prev_t {
                if t < prev {
                    out_of_order += 1;
                }
            }
            prev_t = Some(t);
        }
    }

    IntegrityReport {
        line_number_gaps: gaps,
        out_of_order_timestamps: out_of_order,
        blank_lines: lines.iter().filter(|l| l.raw.trim().is_empty()).count(),
        no_level_lines: lines.iter().filter(|l| l.level.is_none()).count(),
    }
}

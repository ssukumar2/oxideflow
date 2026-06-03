//! Sample log lines deterministically for analysis of large files.

use crate::parser::LogLine;

/// Return every Nth line, starting from index 0.
#[allow(dead_code)]
pub fn every_nth(lines: &[LogLine], n: usize) -> Vec<&LogLine> {
    if n == 0 {
        return Vec::new();
    }
    lines.iter().step_by(n).collect()
}

/// Reservoir sampling: return at most `k` lines uniformly at random
/// using a deterministic seed for reproducibility.
#[allow(dead_code)]
pub fn reservoir(lines: &[LogLine], k: usize, seed: u64) -> Vec<LogLine> {
    if k == 0 || lines.is_empty() {
        return Vec::new();
    }
    let mut state = seed;
    let mut rng = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut reservoir: Vec<LogLine> = lines.iter().take(k).cloned().collect();
    for (i, line) in lines.iter().enumerate().skip(k) {
        let j = (rng() as usize) % (i + 1);
        if j < k {
            reservoir[j] = line.clone();
        }
    }
    reservoir
}

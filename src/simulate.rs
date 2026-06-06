//! Generate synthetic log lines for testing and benchmarking.

use crate::parser::LogLine;

#[allow(dead_code)]
pub struct SimConfig {
    pub count: usize,
    pub error_rate_pct: f64,
    pub warn_rate_pct: f64,
    pub seed: u64,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            count: 1000,
            error_rate_pct: 5.0,
            warn_rate_pct: 10.0,
            seed: 42,
        }
    }
}

#[allow(dead_code)]
pub fn generate(cfg: &SimConfig) -> Vec<LogLine> {
    let mut state = cfg.seed;
    let mut rng = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let messages = [
        "request completed",
        "connection refused",
        "user logged in",
        "cache miss",
        "timeout while reading",
        "database query took 45ms",
        "retry attempt 1",
    ];
    let mut lines = Vec::with_capacity(cfg.count);
    for i in 0..cfg.count {
        let pick = (rng() % 100) as f64;
        let level = if pick < cfg.error_rate_pct {
            "ERROR"
        } else if pick < cfg.error_rate_pct + cfg.warn_rate_pct {
            "WARN"
        } else {
            "INFO"
        };
        let msg = messages[(rng() as usize) % messages.len()];
        let hour = (i / 360) % 24;
        let min = (i / 60) % 60;
        let sec = i % 60;
        let raw = format!(
            "2026-04-16 {:02}:{:02}:{:02} {} {}",
            hour, min, sec, level, msg
        );
        lines.push(LogLine {
            line_number: i + 1,
            level: Some(level.to_string()),
            raw,
        });
    }
    lines
}

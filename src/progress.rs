//! Lightweight progress reporting for long-running operations.

use std::io::Write;
use std::time::Instant;

#[allow(dead_code)]
pub struct ProgressBar {
    total: usize,
    current: usize,
    started: Instant,
    label: String,
}

impl ProgressBar {
    #[allow(dead_code)]
    pub fn new(total: usize, label: &str) -> Self {
        Self {
            total,
            current: 0,
            started: Instant::now(),
            label: label.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn tick(&mut self) {
        self.current += 1;
        if self.current.is_multiple_of(1000) || self.current == self.total {
            self.render();
        }
    }

    fn render(&self) {
        let pct = if self.total == 0 {
            100.0
        } else {
            (self.current as f64 / self.total as f64) * 100.0
        };
        let elapsed = self.started.elapsed().as_secs_f64();
        let rate = if elapsed > 0.0 {
            self.current as f64 / elapsed
        } else {
            0.0
        };
        eprint!(
            "\r{}: {}/{} ({:.1}%) [{:.0} lines/sec]",
            self.label, self.current, self.total, pct, rate
        );
        let _ = std::io::stderr().flush();
    }

    #[allow(dead_code)]
    pub fn finish(&self) {
        self.render();
        eprintln!();
    }
}

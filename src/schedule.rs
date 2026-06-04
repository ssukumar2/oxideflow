//! Schedule periodic re-analysis of a log source.

use std::time::{Duration, Instant};

#[allow(dead_code)]
pub struct Scheduler {
    interval: Duration,
    last_run: Option<Instant>,
}

impl Scheduler {
    #[allow(dead_code)]
    pub fn new(interval_secs: u64) -> Self {
        Self {
            interval: Duration::from_secs(interval_secs),
            last_run: None,
        }
    }

    #[allow(dead_code)]
    pub fn ready(&self) -> bool {
        match self.last_run {
            None => true,
            Some(t) => t.elapsed() >= self.interval,
        }
    }

    #[allow(dead_code)]
    pub fn mark_run(&mut self) {
        self.last_run = Some(Instant::now());
    }

    #[allow(dead_code)]
    pub fn run_if_ready<F>(&mut self, mut f: F) -> bool
    where
        F: FnMut(),
    {
        if self.ready() {
            f();
            self.mark_run();
            true
        } else {
            false
        }
    }
}

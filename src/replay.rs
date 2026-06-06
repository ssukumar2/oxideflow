//! Replay log lines at original timing, optionally accelerated.

use crate::parser::LogLine;

/// Iterator that yields lines with sleep delays matching original timestamps,
/// scaled by `speed` (e.g. 2.0 = twice as fast, 0.5 = half speed).
#[allow(dead_code)]
pub fn replay_with_timing<F>(lines: &[LogLine], speed: f64, mut emit: F)
where
    F: FnMut(&LogLine),
{
    let mut prev: Option<u64> = None;
    for line in lines {
        if let Some(now) = crate::timefilter::parse_time_to_seconds(&line.raw) {
            if let Some(p) = prev {
                let gap = now.saturating_sub(p) as f64 / speed;
                if gap > 0.0 && gap.is_finite() {
                    std::thread::sleep(std::time::Duration::from_secs_f64(gap));
                }
            }
            prev = Some(now);
        }
        emit(line);
    }
}

/// Emit lines at a fixed rate, ignoring original timestamps.
#[allow(dead_code)]
pub fn replay_at_rate<F>(lines: &[LogLine], lines_per_sec: f64, mut emit: F)
where
    F: FnMut(&LogLine),
{
    if lines_per_sec <= 0.0 {
        for line in lines {
            emit(line);
        }
        return;
    }
    let delay = std::time::Duration::from_secs_f64(1.0 / lines_per_sec);
    for line in lines {
        emit(line);
        std::thread::sleep(delay);
    }
}

/// Emit lines in batches with a pause between batches.
#[allow(dead_code)]
pub fn replay_in_batches<F>(lines: &[LogLine], batch_size: usize, pause_secs: f64, mut emit: F)
where
    F: FnMut(&LogLine),
{
    for (i, line) in lines.iter().enumerate() {
        emit(line);
        if (i + 1) % batch_size == 0 {
            std::thread::sleep(std::time::Duration::from_secs_f64(pause_secs));
        }
    }
}

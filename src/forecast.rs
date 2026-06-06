//! Lightweight linear forecasting of log volume.

use crate::parser::LogLine;

/// Fit a linear regression y = mx + b over per-hour line counts and return (slope, intercept).
#[allow(dead_code)]
pub fn linear_trend(lines: &[LogLine]) -> (f64, f64) {
    let buckets = crate::stats::lines_per_hour(lines);
    let n = buckets.len() as f64;
    if n < 2.0 {
        return (0.0, 0.0);
    }
    let xs: Vec<f64> = (0..buckets.len()).map(|i| i as f64).collect();
    let ys: Vec<f64> = buckets.iter().map(|(_, c)| *c as f64).collect();
    let sum_x: f64 = xs.iter().sum();
    let sum_y: f64 = ys.iter().sum();
    let sum_xy: f64 = xs.iter().zip(&ys).map(|(x, y)| x * y).sum();
    let sum_x2: f64 = xs.iter().map(|x| x * x).sum();
    let denom = n * sum_x2 - sum_x * sum_x;
    if denom == 0.0 {
        return (0.0, sum_y / n);
    }
    let slope = (n * sum_xy - sum_x * sum_y) / denom;
    let intercept = (sum_y - slope * sum_x) / n;
    (slope, intercept)
}

/// Predict the line count `hours_ahead` hours from now.
#[allow(dead_code)]
pub fn predict_next(lines: &[LogLine], hours_ahead: usize) -> f64 {
    let (slope, intercept) = linear_trend(lines);
    let buckets = crate::stats::lines_per_hour(lines);
    let x = (buckets.len() + hours_ahead) as f64;
    (slope * x + intercept).max(0.0)
}

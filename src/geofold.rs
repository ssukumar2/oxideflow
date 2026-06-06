//! Fold lines into day-of-week and hour-of-day buckets.

use crate::parser::LogLine;

#[allow(dead_code)]
pub fn fold_by_hour_of_day(lines: &[LogLine]) -> [usize; 24] {
    let re = regex::Regex::new(r"\d{4}-\d{2}-\d{2}[T ](\d{2}):\d{2}:\d{2}").unwrap();
    let mut buckets = [0usize; 24];
    for line in lines {
        if let Some(c) = re.captures(&line.raw) {
            if let Ok(h) = c[1].parse::<usize>() {
                if h < 24 {
                    buckets[h] += 1;
                }
            }
        }
    }
    buckets
}

#[allow(dead_code)]
pub fn peak_hour(lines: &[LogLine]) -> Option<(usize, usize)> {
    let buckets = fold_by_hour_of_day(lines);
    buckets
        .iter()
        .enumerate()
        .max_by_key(|(_, count)| **count)
        .map(|(h, c)| (h, *c))
}

#[allow(dead_code)]
pub fn quiet_hour(lines: &[LogLine]) -> Option<(usize, usize)> {
    let buckets = fold_by_hour_of_day(lines);
    buckets
        .iter()
        .enumerate()
        .filter(|(_, c)| **c > 0)
        .min_by_key(|(_, count)| **count)
        .map(|(h, c)| (h, *c))
}

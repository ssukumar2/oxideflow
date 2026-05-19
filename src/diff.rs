//! Compare two log slices and report differences in level counts.

use crate::parser::LogLine;
use std::collections::HashMap;

#[allow(dead_code)]
pub struct LevelDiff {
    pub level: String,
    pub before: usize,
    pub after: usize,
    pub delta: i64,
}

#[allow(dead_code)]
pub fn compare_level_counts(before: &[LogLine], after: &[LogLine]) -> Vec<LevelDiff> {
    let count = |lines: &[LogLine]| -> HashMap<String, usize> {
        let mut m: HashMap<String, usize> = HashMap::new();
        for l in lines {
            let k = l
                .level
                .clone()
                .unwrap_or_else(|| "UNKNOWN".to_string())
                .to_uppercase();
            *m.entry(k).or_insert(0) += 1;
        }
        m
    };
    let b = count(before);
    let a = count(after);
    let mut keys: std::collections::HashSet<String> = std::collections::HashSet::new();
    keys.extend(b.keys().cloned());
    keys.extend(a.keys().cloned());
    let mut out: Vec<LevelDiff> = keys
        .into_iter()
        .map(|k| {
            let bc = *b.get(&k).unwrap_or(&0);
            let ac = *a.get(&k).unwrap_or(&0);
            LevelDiff {
                level: k,
                before: bc,
                after: ac,
                delta: ac as i64 - bc as i64,
            }
        })
        .collect();
    out.sort_by_key(|d| std::cmp::Reverse(d.delta.abs()));
    out
}

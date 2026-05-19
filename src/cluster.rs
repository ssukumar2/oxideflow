//! Cluster similar log messages by trigram similarity.

use crate::parser::LogLine;
use std::collections::HashSet;

#[allow(dead_code)]
fn trigrams(s: &str) -> HashSet<String> {
    let chars: Vec<char> = s.chars().collect();
    let mut set = HashSet::new();
    if chars.len() < 3 {
        return set;
    }
    for w in chars.windows(3) {
        set.insert(w.iter().collect::<String>());
    }
    set
}

#[allow(dead_code)]
fn similarity(a: &str, b: &str) -> f64 {
    let ta = trigrams(a);
    let tb = trigrams(b);
    if ta.is_empty() || tb.is_empty() {
        return 0.0;
    }
    let inter = ta.intersection(&tb).count() as f64;
    let union = ta.union(&tb).count() as f64;
    inter / union
}

#[allow(dead_code)]
pub fn cluster_messages(lines: &[LogLine], threshold: f64) -> Vec<Vec<String>> {
    let mut clusters: Vec<Vec<String>> = Vec::new();
    for line in lines {
        let mut placed = false;
        for cluster in clusters.iter_mut() {
            if similarity(&cluster[0], &line.raw) >= threshold {
                cluster.push(line.raw.clone());
                placed = true;
                break;
            }
        }
        if !placed {
            clusters.push(vec![line.raw.clone()]);
        }
    }
    clusters
}

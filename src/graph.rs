//! A directed graph of log-level transitions with force-directed layout.

use std::collections::HashMap;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub x: f64,
    pub y: f64,
    pub weight: usize,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub weight: usize,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl Graph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
    pub fn add_node(&mut self, id: &str, weight: usize) {
        if self.nodes.iter().any(|n| n.id == id) {
            return;
        }
        self.nodes.push(Node {
            id: id.to_string(),
            label: id.to_string(),
            x: 0.0,
            y: 0.0,
            weight,
        });
    }

    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: &str, to: &str, weight: usize) {
        self.edges.push(Edge {
            from: from.to_string(),
            to: to.to_string(),
            weight,
        });
    }

    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    #[allow(dead_code)]
    pub fn neighbors(&self, id: &str) -> Vec<&Node> {
        let neighbor_ids: Vec<&str> = self
            .edges
            .iter()
            .filter_map(|e| {
                if e.from == id {
                    Some(e.to.as_str())
                } else if e.to == id {
                    Some(e.from.as_str())
                } else {
                    None
                }
            })
            .collect();
        self.nodes
            .iter()
            .filter(|n| neighbor_ids.contains(&n.id.as_str()))
            .collect()
    }
}

/// Build a graph from log-level transition counts.
#[allow(dead_code)]
pub fn from_level_transitions(transitions: &[((String, String), usize)]) -> Graph {
    let mut g = Graph::new();
    let mut node_weights: HashMap<String, usize> = HashMap::new();
    for ((from, to), count) in transitions {
        *node_weights.entry(from.clone()).or_insert(0) += count;
        *node_weights.entry(to.clone()).or_insert(0) += count;
    }
    for (id, weight) in &node_weights {
        g.add_node(id, *weight);
    }
    for ((from, to), count) in transitions {
        g.add_edge(from, to, *count);
    }
    g
}

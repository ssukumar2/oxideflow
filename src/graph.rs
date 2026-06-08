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

/// Apply Fruchterman-Reingold force-directed layout.
/// Nodes repel each other, edges pull connected nodes together.
/// Runs for `iterations` steps within an `area` x `area` square.
#[allow(dead_code)]
pub fn force_directed_layout(graph: &mut Graph, area: f64, iterations: usize) {
    let n = graph.nodes.len();
    if n == 0 {
        return;
    }

    // Initial random-ish placement using node index for determinism
    for (i, node) in graph.nodes.iter_mut().enumerate() {
        let angle = (i as f64) * 2.4;
        let radius = area / 3.0;
        node.x = area / 2.0 + radius * angle.cos();
        node.y = area / 2.0 + radius * angle.sin();
    }

    let k = (area * area / n as f64).sqrt();
    let mut temperature = area / 10.0;
    let cooling = temperature / iterations as f64;

    for _ in 0..iterations {
        let mut displacements: Vec<(f64, f64)> = vec![(0.0, 0.0); n];

        // Repulsive forces between every pair of nodes
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let dx = graph.nodes[i].x - graph.nodes[j].x;
                let dy = graph.nodes[i].y - graph.nodes[j].y;
                let dist = (dx * dx + dy * dy).sqrt().max(0.01);
                let force = k * k / dist;
                displacements[i].0 += (dx / dist) * force;
                displacements[i].1 += (dy / dist) * force;
            }
        }

        // Attractive forces along edges
        for edge in &graph.edges {
            let from_idx = graph.nodes.iter().position(|n| n.id == edge.from);
            let to_idx = graph.nodes.iter().position(|n| n.id == edge.to);
            if let (Some(i), Some(j)) = (from_idx, to_idx) {
                let dx = graph.nodes[i].x - graph.nodes[j].x;
                let dy = graph.nodes[i].y - graph.nodes[j].y;
                let dist = (dx * dx + dy * dy).sqrt().max(0.01);
                let force = (dist * dist) / k;
                let fx = (dx / dist) * force;
                let fy = (dy / dist) * force;
                displacements[i].0 -= fx;
                displacements[i].1 -= fy;
                displacements[j].0 += fx;
                displacements[j].1 += fy;
            }
        }

        // Apply displacements, capped by temperature, kept inside bounds
        for (i, node) in graph.nodes.iter_mut().enumerate() {
            let (dx, dy) = displacements[i];
            let mag = (dx * dx + dy * dy).sqrt().max(0.01);
            let capped = mag.min(temperature);
            node.x = (node.x + (dx / mag) * capped).clamp(20.0, area - 20.0);
            node.y = (node.y + (dy / mag) * capped).clamp(20.0, area - 20.0);
        }

        temperature -= cooling;
        temperature = temperature.max(1.0);
    }
}

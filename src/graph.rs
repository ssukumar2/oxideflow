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

/// Render a laid-out graph as standalone SVG.
/// Node size scales with weight, edge stroke-width scales with weight.
#[allow(dead_code)]
pub fn render_svg(graph: &Graph, width: f64, height: f64) -> String {
    let max_node_weight = graph
        .nodes
        .iter()
        .map(|n| n.weight)
        .max()
        .unwrap_or(1)
        .max(1);
    let max_edge_weight = graph
        .edges
        .iter()
        .map(|e| e.weight)
        .max()
        .unwrap_or(1)
        .max(1);

    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" font-family=\"sans-serif\">",
        width, height
    );

    svg.push_str(
        "<defs><marker id=\"arrow\" viewBox=\"0 0 10 10\" refX=\"8\" refY=\"5\"          markerWidth=\"6\" markerHeight=\"6\" orient=\"auto\">         <path d=\"M0,0 L10,5 L0,10 z\" fill=\"#888\"/></marker></defs>",
    );

    // Edges first so nodes overlay them
    for edge in &graph.edges {
        let from = graph.nodes.iter().find(|n| n.id == edge.from);
        let to = graph.nodes.iter().find(|n| n.id == edge.to);
        if let (Some(f), Some(t)) = (from, to) {
            let stroke = 1.0 + 4.0 * (edge.weight as f64 / max_edge_weight as f64);
            svg.push_str(&format!(
                "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\"                  stroke=\"#888\" stroke-width=\"{:.1}\"                  marker-end=\"url(#arrow)\" opacity=\"0.7\"/>",
                f.x, f.y, t.x, t.y, stroke
            ));
        }
    }

    // Nodes with labels
    for node in &graph.nodes {
        let radius = 15.0 + 25.0 * (node.weight as f64 / max_node_weight as f64);
        let color = match node.label.to_uppercase().as_str() {
            "ERROR" => "#e74c3c",
            "WARN" | "WARNING" => "#f39c12",
            "INFO" => "#3498db",
            "DEBUG" => "#95a5a6",
            "TRACE" => "#8e44ad",
            _ => "#34495e",
        };
        svg.push_str(&format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"{}\"              stroke=\"white\" stroke-width=\"2\"/>",
            node.x, node.y, radius, color
        ));
        svg.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\"              dominant-baseline=\"middle\" fill=\"white\" font-size=\"12\"              font-weight=\"bold\">{}</text>",
            node.x, node.y, node.label
        ));
    }

    svg.push_str("</svg>");
    svg
}

/// Convenience: build, layout, and render a level-transition graph in one call.
#[allow(dead_code)]
pub fn level_transition_svg(lines: &[crate::parser::LogLine], width: f64, height: f64) -> String {
    let transitions = crate::stats::level_transitions(lines);
    let mut g = from_level_transitions(&transitions);
    force_directed_layout(&mut g, width.min(height), 100);
    render_svg(&g, width, height)
}

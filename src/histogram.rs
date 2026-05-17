//! ASCII histogram rendering for tallied data.

/// Render a horizontal bar chart from (label, count) pairs.
/// Bars scale to fit within `max_width` characters.
#[allow(dead_code)]
pub fn render(data: &[(String, usize)], max_width: usize) -> String {
    if data.is_empty() {
        return String::new();
    }
    let max_count = data.iter().map(|(_, c)| *c).max().unwrap_or(1);
    let max_label = data.iter().map(|(l, _)| l.len()).max().unwrap_or(0);
    let mut out = String::new();
    for (label, count) in data {
        let bar_len = if max_count == 0 {
            0
        } else {
            (count * max_width) / max_count
        };
        let bar = "#".repeat(bar_len);
        out.push_str(&format!(
            "{:<width$}  {} ({})\n",
            label,
            bar,
            count,
            width = max_label
        ));
    }
    out
}

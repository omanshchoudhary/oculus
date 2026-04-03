use crate::types::Stats;

pub fn render_table(stats: &Stats) -> String {
    let mut out = String::new();

    out.push_str("=== Summary ===\n");
    out.push_str(&format!("Total lines: {}\n", stats.total_lines));
    out.push_str(&format!("Parsed lines: {}\n", stats.parsed_lines));
    out.push_str(&format!("Parse errors: {}\n", stats.parsed_errors));

    out.push_str("\nStatus counts:\n");
    let mut status_items: Vec<(u16, usize)> =
        stats.status_counts.iter().map(|(k, v)| (*k, *v)).collect();
    status_items.sort_by_key(|(code, _)| *code);

    for (code, count) in status_items {
        out.push_str(&format!("  {} -> {}\n", code, count));
    }

    out.push_str("\nTop paths:\n");
    for (path, count) in stats.top_paths_sorted(10) {
        out.push_str(&format!("  {} -> {}\n", path, count));
    }

    out
}

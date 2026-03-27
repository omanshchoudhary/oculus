use crate::types::Stats;

pub fn print_summary(stats: &Stats) {
    println!("=== Summary ===");
    println!("Total lines: {}", stats.total_lines);
    println!("Parsed lines: {}", stats.parsed_lines);
    println!("Parse errors: {}", stats.parsed_errors);

    println!("\nStatus counts:");
    let mut status_items: Vec<(u16, usize)> =
        stats.status_counts.iter().map(|(k, v)| (*k, *v)).collect();
    status_items.sort_by_key(|(code, _)| *code);

    for (code, count) in status_items {
        println!("  {} -> {}", code, count);
    }

    println!("\nTop paths:");
    for (path, count) in stats.top_paths_sorted(10) {
        println!("  {} -> {}", path, count);
    }
}

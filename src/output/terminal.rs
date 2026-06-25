use crate::output::Report;

pub fn render_table(report: &Report) -> String {
    let mut out = String::new();

    out.push_str("=== Summary ===\n");
    out.push_str(&format!("Total lines: {}\n", report.total_lines));
    out.push_str(&format!("Parsed lines: {}\n", report.parsed_lines));
    out.push_str(&format!("Parse errors: {}\n", report.parse_errors));

    out.push_str("\nStatus counts:\n");
    for (code, count) in &report.status_counts {
        out.push_str(&format!("  {} -> {}\n", code, count));
    }

    out.push_str("\nTop paths:\n");
    for (path, count) in &report.top_paths {
        out.push_str(&format!("  {} -> {}\n", path, count));
    }

    out
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Stats;

    #[test]
    fn renders_table_summary() {
        let mut stats = Stats {
            total_lines: 5,
            parsed_lines: 4,
            parse_errors: 1,
            ..Stats::default()
        };
        stats.status_counts.insert(500, 1);
        stats.status_counts.insert(200, 3);

        let report = Report::from_stats(&stats);
        let rendered = render_table(&report);

        assert!(rendered.contains("Total lines: 5"));
        assert!(rendered.contains("Parsed lines: 4"));
        assert!(rendered.contains("Parse errors: 1"));
        assert!(rendered.contains("200 -> 3"));
        assert!(rendered.contains("500 -> 1"));
        // status codes are rendered in ascending order
        assert!(rendered.find("200 -> 3").unwrap() < rendered.find("500 -> 1").unwrap());
    }
}

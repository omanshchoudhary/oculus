use crate::output::Report;

// ANSI escape codes used to style and add color to text in the terminal
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";

fn paint(text: &str, code: &str, use_color: bool) -> String {
    if use_color {
        format!("{code}{text}{RESET}")
    } else {
        text.to_string()
    }
}

fn status_color(code: u16) -> &'static str {
    match code {
        200..=299 => GREEN,
        300..=399 => YELLOW,
        _ => RED,
    }
}

pub fn render_table(report: &Report, use_color: bool) -> String {
    let mut out = String::new();

    out.push_str(&paint("=== Summary ===", BOLD, use_color));
    out.push('\n');

    out.push_str(&format!("Total lines: {}\n", report.total_lines));
    out.push_str(&format!("Parsed lines: {}\n", report.parsed_lines));
    out.push_str(&format!("Parse errors: {}\n", report.parse_errors));

    out.push_str(&paint("\nStatus counts:", BOLD, use_color));
    out.push('\n');

    for (code, count) in &report.status_counts {
        let painted_code = paint(&code.to_string(), status_color(*code), use_color);
        out.push_str(&format!("  {} -> {}\n", painted_code, count));
    }

    out.push_str(&paint("\nTop paths:", BOLD, use_color));
    out.push('\n');
    for (path, count) in &report.top_paths {
        out.push_str(&format!("  {} -> {}\n", path, count));
    }

    if !report.error_samples.is_empty() {
        let heading = format!(
            "\nParse errors (showing {} of {}):",
            report.error_samples.len(),
            report.parse_errors
        );
        out.push_str(&paint(&heading, BOLD, use_color));
        out.push('\n');
        for (line_no, reason) in &report.error_samples {
            out.push_str(&format!("  line {}: {}\n", line_no, reason));
        }
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
        let rendered = render_table(&report, false);

        assert!(rendered.contains("Total lines: 5"));
        assert!(rendered.contains("Parsed lines: 4"));
        assert!(rendered.contains("Parse errors: 1"));
        assert!(rendered.contains("200 -> 3"));
        assert!(rendered.contains("500 -> 1"));
        // status codes are rendered in ascending order
        assert!(rendered.find("200 -> 3").unwrap() < rendered.find("500 -> 1").unwrap());
        // plain mode must not emit any ANSI escape codes
        assert!(!rendered.contains('\x1b'));
    }

    #[test]
    fn renders_color_when_enabled() {
        let mut stats = Stats {
            total_lines: 1,
            parsed_lines: 1,
            parse_errors: 0,
            ..Stats::default()
        };
        stats.status_counts.insert(200, 1);

        let report = Report::from_stats(&stats);
        let rendered = render_table(&report, true);

        // color mode wraps text in ANSI escape codes
        assert!(rendered.contains('\x1b'));
    }

    #[test]
    fn render_table_snapshot() {
        let mut stats = Stats {
            total_lines: 10,
            parsed_lines: 8,
            parse_errors: 2,
            ..Stats::default()
        };
        stats.status_counts.insert(200, 5);
        stats.status_counts.insert(404, 2);
        stats.status_counts.insert(500, 1);
        stats.top_paths.insert("/api/users".to_string(), 4);
        stats.top_paths.insert("/health".to_string(), 3);
        stats
            .error_samples
            .push((2, "invalid apache line".to_string()));
        stats
            .error_samples
            .push((7, "invalid apache line".to_string()));

        let report = Report::from_stats(&stats);
        // plain mode keeps the snapshot free of ANSI escape codes
        insta::assert_snapshot!(render_table(&report, false));
    }
}

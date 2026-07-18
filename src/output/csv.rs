use crate::output::Report;

pub fn render_csv(report: &Report) -> String {
    let mut out = String::new();
    out.push_str("metric,value\n");
    out.push_str(&format!("total_lines,{}\n", report.total_lines));
    out.push_str(&format!("parsed_lines,{}\n", report.parsed_lines));
    out.push_str(&format!("parse_errors,{}\n", report.parse_errors));

    for (code, count) in &report.status_counts {
        out.push_str(&format!("status_{},{}\n", code, count));
    }

    for (line_no, reason) in &report.error_samples {
        // Quote the reason and escape embedded quotes so commas stay safe.
        let escaped = reason.replace('"', "\"\"");
        out.push_str(&format!("error_line_{},\"{}\"\n", line_no, escaped));
    }

    out
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Stats;

    #[test]
    fn renders_csv_summary() {
        let mut stats = Stats {
            total_lines: 5,
            parsed_lines: 4,
            parse_errors: 1,
            ..Stats::default()
        };
        stats.status_counts.insert(200, 3);
        stats.status_counts.insert(500, 1);

        let report = Report::from_stats(&stats);
        let rendered = render_csv(&report);

        assert!(rendered.contains("metric,value"));
        assert!(rendered.contains("total_lines,5"));
        assert!(rendered.contains("parsed_lines,4"));
        assert!(rendered.contains("parse_errors,1"));
        assert!(rendered.contains("status_200,3"));
        assert!(rendered.contains("status_500,1"));
    }

    #[test]
    fn escapes_quotes_in_error_samples() {
        // regression: embedded quotes must be doubled per the csv convention
        let mut stats = Stats {
            total_lines: 1,
            parsed_lines: 0,
            parse_errors: 1,
            ..Stats::default()
        };
        stats
            .error_samples
            .push((1, r#"unexpected "quote" here"#.to_string()));

        let report = Report::from_stats(&stats);
        let rendered = render_csv(&report);

        assert!(rendered.contains(r#"error_line_1,"unexpected ""quote"" here""#));
    }
}

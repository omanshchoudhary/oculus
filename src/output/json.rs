use crate::output::Report;
use anyhow::Result;

pub fn render_json(report: &Report) -> Result<String> {
    Ok(serde_json::to_string_pretty(report)?)
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Stats;

    #[test]
    fn renders_json_summary() {
        let mut stats = Stats {
            total_lines: 3,
            parsed_lines: 2,
            parse_errors: 1,
            ..Stats::default()
        };
        stats.status_counts.insert(200, 1);
        stats.status_counts.insert(404, 1);
        stats.top_paths.insert("/a".to_string(), 1);
        stats.top_paths.insert("/b".to_string(), 1);

        let report = Report::from_stats(&stats);
        let rendered = render_json(&report).expect("json render succeeds");

        assert!(rendered.contains("\"total_lines\": 3"));
        assert!(rendered.contains("\"parsed_lines\": 2"));
        assert!(rendered.contains("\"parse_errors\": 1"));
    }
}

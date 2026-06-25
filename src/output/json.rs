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

    #[test]
    fn json_output_matches_expected_schema() {
        let mut stats = Stats {
            total_lines: 4,
            parsed_lines: 3,
            parse_errors: 1,
            ..Stats::default()
        };
        stats.status_counts.insert(200, 3);
        stats.top_paths.insert("/api".to_string(), 3);
        stats.error_samples.push((4, "invalid line".to_string()));

        let report = Report::from_stats(&stats);
        let rendered = render_json(&report).expect("json render succeeds");
        let value: serde_json::Value =
            serde_json::from_str(&rendered).expect("output is valid json");

        // every expected key is present with the right JSON type
        assert!(value["total_lines"].is_u64());
        assert!(value["parsed_lines"].is_u64());
        assert!(value["parse_errors"].is_u64());
        assert!(value["status_counts"].is_array());
        assert!(value["top_paths"].is_array());
        assert!(value["error_samples"].is_array());

        // values round-trip correctly
        assert_eq!(value["total_lines"], 4);
        assert_eq!(value["parsed_lines"], 3);
        assert_eq!(value["parse_errors"], 1);
        assert_eq!(value["status_counts"][0][0], 200);
        assert_eq!(value["status_counts"][0][1], 3);
        assert_eq!(value["error_samples"][0][0], 4);
        assert_eq!(value["error_samples"][0][1], "invalid line");
    }
}

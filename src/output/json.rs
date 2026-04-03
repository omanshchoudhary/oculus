use crate::types::Stats;
use anyhow::Result;
use serde::Serialize;

#[derive(Serialize)]
struct JsonSummary {
    total_lines: usize,
    parsed_lines: usize,
    parse_errors: usize,
    status_counts: Vec<(u16, usize)>,
    top_paths: Vec<(String, usize)>,
}

pub fn render_json(stats: &Stats) -> Result<String> {
    let mut status_counts: Vec<(u16, usize)> =
        stats.status_counts.iter().map(|(k, v)| (*k, *v)).collect();
    status_counts.sort_by_key(|(code, _)| *code);

    let payload = JsonSummary {
        total_lines: stats.total_lines,
        parsed_lines: stats.parsed_lines,
        parse_errors: stats.parsed_errors,
        status_counts,
        top_paths: stats.top_paths_sorted(10),
    };

    Ok(serde_json::to_string_pretty(&payload)?)
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
            parsed_errors: 1,
            ..Stats::default()
        };
        stats.status_counts.insert(200, 1);
        stats.status_counts.insert(404, 1);
        stats.top_paths.insert("/a".to_string(), 1);
        stats.top_paths.insert("/b".to_string(), 1);

        let rendered = render_json(&stats).expect("json render succeeds");

        assert!(rendered.contains("\"total_lines\": 3"));
        assert!(rendered.contains("\"parsed_lines\": 2"));
        assert!(rendered.contains("\"parse_errors\": 1"));
    }
}

use crate::types::Stats;

pub fn render_csv(stats: &Stats) -> String {
    let mut out = String::new();
    out.push_str("metric,value\n");
    out.push_str(&format!("total_lines,{}\n", stats.total_lines));
    out.push_str(&format!("parsed_lines,{}\n", stats.parsed_lines));
    out.push_str(&format!("parse_errors,{}\n", stats.parsed_errors));

    let mut status_counts: Vec<(u16, usize)> =
        stats.status_counts.iter().map(|(k, v)| (*k, *v)).collect();
    status_counts.sort_by_key(|(code, _)| *code);

    for (code, count) in status_counts {
        out.push_str(&format!("status_{},{}\n", code, count));
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
            parsed_errors: 1,
            ..Stats::default()
        };
        stats.status_counts.insert(200, 3);
        stats.status_counts.insert(500, 1);

        let rendered = render_csv(&stats);

        assert!(rendered.contains("metric,value"));
        assert!(rendered.contains("total_lines,5"));
        assert!(rendered.contains("parsed_lines,4"));
        assert!(rendered.contains("parse_errors,1"));
        assert!(rendered.contains("status_200,3"));
        assert!(rendered.contains("status_500,1"));
    }
}

use crate::types::{LogEntry, Stats};
const MAX_ERROR_SAMPLES: usize = 5;
impl Stats {
    pub fn on_line_read(&mut self) {
        self.total_lines += 1;
    }

    pub fn on_parsed_entry(&mut self, entry: &LogEntry) {
        self.parsed_lines += 1;

        if let Some(code) = entry.status {
            *self.status_counts.entry(code).or_insert(0) += 1;
        }

        if let Some(path) = entry.path.as_ref() {
            *self.top_paths.entry(path.clone()).or_insert(0) += 1;
        }
    }

    pub fn on_parse_error(&mut self, message: &str) {
        self.parse_errors += 1;
        if self.error_samples.len() < MAX_ERROR_SAMPLES {
            self.error_samples
                .push((self.total_lines, message.to_string()));
        };
    }

    pub fn top_paths_sorted(&self, limit: usize) -> Vec<(String, usize)> {
        let mut items: Vec<(String, usize)> = self
            .top_paths
            .iter()
            .map(|(path, count)| (path.clone(), *count))
            .collect();

        items.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        items.truncate(limit);
        items
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use crate::types::Stats;

    #[test]
    fn parse_error_records_current_line_number() {
        // regression: samples must carry the line number the error occurred on
        let mut stats = Stats::default();
        stats.on_line_read(); // line 1
        stats.on_line_read(); // line 2
        stats.on_parse_error("bad line");

        assert_eq!(stats.error_samples, vec![(2, "bad line".to_string())]);
    }

    #[test]
    fn error_samples_are_capped_but_count_keeps_growing() {
        // regression: the sample list is bounded, the error count is not
        let mut stats = Stats::default();
        for _ in 0..10 {
            stats.on_line_read();
            stats.on_parse_error("bad");
        }

        assert_eq!(stats.parse_errors, 10);
        assert_eq!(stats.error_samples.len(), 5);
    }
}

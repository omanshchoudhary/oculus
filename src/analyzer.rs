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

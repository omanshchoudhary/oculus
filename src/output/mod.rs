use crate::types::Stats;
use serde::Serialize;

pub mod csv;
pub mod json;
pub mod terminal;

#[derive(Serialize, Debug, Clone)]
pub struct Report {
    pub total_lines: usize,
    pub parsed_lines: usize,
    pub parse_errors: usize,
    pub status_counts: Vec<(u16, usize)>, // sorted ascending by status code
    pub top_paths: Vec<(String, usize)>,  // sorted from highest to lowest count
}

impl Report {
    pub fn from_stats(stats: &Stats) -> Self {
        let mut status_counts: Vec<(u16, usize)> =
            stats.status_counts.iter().map(|(k, v)| (*k, *v)).collect();
        status_counts.sort_by_key(|(code, _)| *code);

        Self {
            total_lines: stats.total_lines,
            parsed_lines: stats.parsed_lines,
            parse_errors: stats.parse_errors,
            status_counts,
            top_paths: stats.top_paths_sorted(10),
        }
    }
}

use std::collections::HashMap;


// Modularizing The Logs And Giving Format
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub ip : Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status: Option<u16>,
    pub message: String,
    pub raw: String,
}

#[derive(Debug, Default)]
pub struct Stats {
    pub total_lines: usize,
    pub parsed_lines: usize,
    pub parsed_errors: usize,
    pub status_counts: HashMap<u16,usize>,
}
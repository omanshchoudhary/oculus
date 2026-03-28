use clap::ValueEnum;
use std::collections::HashMap;

// Modularizing The Logs And Giving Format
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub ip: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status: Option<u16>,
    pub message: String,
    pub raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LogFormat {
    Auto,
    Apache,
    Nginx,
    Json,
}

#[derive(Debug, Default)]
pub struct Stats {
    pub total_lines: usize,
    pub parsed_lines: usize,
    pub parsed_errors: usize,
    pub status_counts: HashMap<u16, usize>,
    pub top_paths: HashMap<String, usize>,
}

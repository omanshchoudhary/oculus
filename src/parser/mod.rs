use crate::types::LogEntry;

// Creating a trait to be followed by the parsers
pub trait LogParser {
    fn parse(&self, line: &str) -> Result<LogEntry, String>;
}

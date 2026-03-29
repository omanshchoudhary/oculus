use crate::types::LogEntry;
pub mod apache;
pub mod detector;
pub mod json;
pub mod nginx;
// Creating a trait to be followed by the parsers
pub trait LogParser {
    fn parse(&self, line: &str) -> Result<LogEntry, String>;
}

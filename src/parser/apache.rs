use regex::Regex;
use crate::parser::LogParser;
use crate::types::LogEntry;

pub struct ApacheParser {
    pub re: regex,
}

impl ApacheParser {
    pub fn new() -> Self {
        // Regex Pattern For Apache Logs Format
        let re = Regex::new(r#"^(\S+) \S+ \S+ \[[^\]]+\] "(\S+) (\S+) [^"]+" (\d{3})"#).expect("valid regex");
        Self {re}
    }
}

impl LogParser for ApacheParser {
    fn parse(&self, line: &str) -> Result<LogEntry, String> {
        let caps = self.re.captures(line).ok_or_else(|| "invalid apache line".to_string())?;

        let status = caps.get(4).and_then(|m| m.as_str().parse::<u16>().ok());

        Ok(LogEntry {
            ip: caps.get(1).map(|m| m.as_str().to_string()),
            method: caps.get(2).map(|m| m.as_str().to_string()),
            path: caps.get(3).map(|m| m.as_str().to_string()),
            status,
            message: String::new(),
            raw: line.to_string(),
        })
    }
}
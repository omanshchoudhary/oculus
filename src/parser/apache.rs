use crate::parser::LogParser;
use crate::types::LogEntry;
use chrono::DateTime;
use regex::Regex;

pub struct ApacheParser {
    pub re: Regex,
}

impl ApacheParser {
    pub fn new() -> Self {
        let re = Regex::new(
            r#"^(?P<ip>\S+) \S+ \S+ \[(?P<ts>[^\]]+)\] "(?P<method>\S+) (?P<path>\S+) [^"]+" (?P<status>\d{3})"#,
        )
        .expect("valid regex");
        Self { re }
    }
}

impl LogParser for ApacheParser {
    fn parse(&self, line: &str) -> Result<LogEntry, String> {
        let caps = self
            .re
            .captures(line)
            .ok_or_else(|| "invalid apache line".to_string())?;

        let status = caps
            .name("status")
            .and_then(|m| m.as_str().parse::<u16>().ok());
        let timestamp = caps
            .name("ts")
            .and_then(|m| DateTime::parse_from_str(m.as_str(), "%d/%b/%Y:%H:%M:%S %z").ok());

        Ok(LogEntry {
            ip: caps.name("ip").map(|m| m.as_str().to_string()),
            method: caps.name("method").map(|m| m.as_str().to_string()),
            path: caps.name("path").map(|m| m.as_str().to_string()),
            status,
            timestamp,
            message: String::new(),
            raw: line.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::LogParser;

    #[test]
    fn parse_valid_apache_line() {
        let parser = ApacheParser::new();
        let line =
            r#"127.0.0.1 - - [10/Oct/2023:13:55:36 +0000] "GET /api/users HTTP/1.1" 200 1234"#;

        let entry = parser.parse(line).expect("should parse");
        assert_eq!(entry.ip.as_deref(), Some("127.0.0.1"));
        assert_eq!(entry.method.as_deref(), Some("GET"));
        assert_eq!(entry.path.as_deref(), Some("/api/users"));
        assert_eq!(entry.status, Some(200));
    }

    #[test]
    fn parse_invalid_apache_line() {
        let parser = ApacheParser::new();
        let bad = "this is not apache log format";
        let res = parser.parse(bad);
        assert!(res.is_err());
    }
}

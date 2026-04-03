use crate::parser::LogParser;
use crate::types::LogEntry;
use chrono::DateTime;
use regex::Regex;

#[allow(dead_code)]
pub struct NginxParser {
    pub re: Regex,
}
#[allow(dead_code)]
impl NginxParser {
    pub fn new() -> Self {
        let re = Regex::new(
            r#"^(?P<ip>\S+) - - \[(?P<ts>[^\]]+)\] "(?P<method>\S+) (?P<path>\S+) [^"]+" (?P<status>\d{3})"#,
        )
        .expect("valid nginx regex");
        Self { re }
    }
}

impl LogParser for NginxParser {
    fn parse(&self, line: &str) -> Result<LogEntry, String> {
        let caps = self
            .re
            .captures(line)
            .ok_or_else(|| "invalid nginx line".to_string())?;

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
    fn parse_valid_nginx_line() {
        let parser = NginxParser::new();
        let line = r#"127.0.0.1 - - [10/Oct/2023:13:55:36 +0000] "GET /health HTTP/1.1" 200 612"#;

        let entry = parser.parse(line).expect("should parse");

        assert_eq!(entry.ip.as_deref(), Some("127.0.0.1"));
        assert_eq!(entry.method.as_deref(), Some("GET"));
        assert_eq!(entry.path.as_deref(), Some("/health"));
        assert_eq!(entry.status, Some(200));
    }

    #[test]
    fn parse_invalid_nginx_line() {
        let parser = NginxParser::new();
        let bad = "not a nginx log line";

        let result = parser.parse(bad);

        assert!(result.is_err());
    }
}

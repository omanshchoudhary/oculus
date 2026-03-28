use crate::parser::LogParser;
use crate::types::LogEntry;
use regex::Regex;

#[allow(dead_code)]
pub struct NginxParser {
    pub re: Regex,
}
#[allow(dead_code)]
impl NginxParser {
    pub fn new() -> Self {
        let re = Regex::new(r#"^(\S+) - - \[[^\]]+\] "(\S+) (\S+) [^"]+" (\d{3})"#)
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

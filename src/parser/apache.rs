use crate::parser::LogParser;
use crate::types::LogEntry;
use regex::Regex;

pub struct ApacheParser {
    pub re: Regex,
}

impl ApacheParser {
    pub fn new() -> Self {
        // Regex Pattern For Apache Logs Format
        let re = Regex::new(r#"^(\S+) \S+ \S+ \[[^\]]+\] "(\S+) (\S+) [^"]+" (\d{3})"#)
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

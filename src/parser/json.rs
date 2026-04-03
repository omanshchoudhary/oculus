use crate::parser::LogParser;
use crate::types::LogEntry;
use chrono::DateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct JsonLogEntry {
    ip: Option<String>,
    method: Option<String>,
    path: Option<String>,
    status: Option<u16>,
    timestamp: Option<String>,
    message: Option<String>,
}

#[allow(dead_code)]
pub struct JsonParser;

#[allow(dead_code)]
impl JsonParser {
    pub fn new() -> Self {
        Self
    }
}

impl LogParser for JsonParser {
    fn parse(&self, line: &str) -> Result<LogEntry, String> {
        let parsed: JsonLogEntry =
            serde_json::from_str(line).map_err(|_| "invalid json log line".to_string())?;

        Ok(LogEntry {
            ip: parsed.ip,
            method: parsed.method,
            path: parsed.path,
            status: parsed.status,
            timestamp: parsed
                .timestamp
                .and_then(|ts| DateTime::parse_from_rfc3339(&ts).ok()),
            message: parsed.message.unwrap_or_default(),
            raw: line.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::LogParser;

    #[test]
    fn parse_valid_json_line() {
        let parser = JsonParser::new();
        let line =
            r#"{"ip":"127.0.0.1","method":"GET","path":"/api/users","status":200,"message":"ok"}"#;

        let entry = parser.parse(line).expect("should parse");

        assert_eq!(entry.ip.as_deref(), Some("127.0.0.1"));
        assert_eq!(entry.method.as_deref(), Some("GET"));
        assert_eq!(entry.path.as_deref(), Some("/api/users"));
        assert_eq!(entry.status, Some(200));
        assert_eq!(entry.message, "ok");
    }

    #[test]
    fn parse_valid_json_line_with_missing_optional_fields() {
        let parser = JsonParser::new();
        let line = r#"{"path":"/health","status":204}"#;

        let entry = parser.parse(line).expect("should parse");

        assert_eq!(entry.ip, None);
        assert_eq!(entry.method, None);
        assert_eq!(entry.path.as_deref(), Some("/health"));
        assert_eq!(entry.status, Some(204));
        assert_eq!(entry.message, "");
    }

    #[test]
    fn parse_invalid_json_line() {
        let parser = JsonParser::new();
        let bad = r#"{"ip":"127.0.0.1","status":"oops""#;

        let result = parser.parse(bad);

        assert!(result.is_err());
    }
}

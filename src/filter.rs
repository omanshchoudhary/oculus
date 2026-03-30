use anyhow::{Ok, Result};
use regex::Regex;

use crate::types::LogEntry;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FilterConfig {
    pub status: Option<u16>,
    pub contains: Option<String>,
    pub regex: Option<String>,
}

#[allow(dead_code)]
pub struct FilterEngine {
    status: Option<u16>,
    contains: Option<String>,
    regex: Option<Regex>,
}

#[allow(dead_code)]
impl FilterEngine {
    pub fn new(config: FilterConfig) -> Result<Self> {
        let compiled_regex = match config.regex {
            Some(pattern) => Some(Regex::new(&pattern)?),
            None => None,
        };

        Ok(Self {
            status: config.status,
            contains: config.contains,
            regex: compiled_regex,
        })
    }
    pub fn accept(&self, entry: &LogEntry) -> bool {
        if let Some(status) = self.status
            && entry.status != Some(status)
        {
            return false;
        }
        if let Some(contains) = self.contains.as_deref()
            && !entry.raw.contains(contains)
        {
            return false;
        }
        if let Some(regex) = &self.regex
            && !regex.is_match(&entry.raw)
        {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LogEntry;

    fn sample_entry() -> LogEntry {
        LogEntry {
            ip: Some("127.0.0.1".to_string()),
            method: Some("GET".to_string()),
            path: Some("/api/users".to_string()),
            status: Some(200),
            message: String::new(),
            raw: r#"127.0.0.1 - - [10/Oct/2023:13:55:36 +0000] "GET /api/users HTTP/1.1" 200 1234"#
                .to_string(),
        }
    }

    #[test]
    fn default_filter_accepts_entry() {
        let engine = FilterEngine::new(FilterConfig::default()).expect("valid filter config");

        assert!(engine.accept(&sample_entry()));
    }

    #[test]
    fn matching_status_is_accepted() {
        let engine = FilterEngine::new(FilterConfig {
            status: Some(200),
            contains: None,
            regex: None,
        })
        .expect("valid filter config");

        assert!(engine.accept(&sample_entry()));
    }

    #[test]
    fn non_matching_status_is_rejected() {
        let engine = FilterEngine::new(FilterConfig {
            status: Some(404),
            contains: None,
            regex: None,
        })
        .expect("valid filter config");

        assert!(!engine.accept(&sample_entry()));
    }

    #[test]
    fn matching_contains_is_accepted() {
        let engine = FilterEngine::new(FilterConfig {
            status: None,
            contains: Some("/api/users".to_string()),
            regex: None,
        })
        .expect("valid filter config");

        assert!(engine.accept(&sample_entry()));
    }

    #[test]
    fn non_matching_contains_is_rejected() {
        let engine = FilterEngine::new(FilterConfig {
            status: None,
            contains: Some("/admin".to_string()),
            regex: None,
        })
        .expect("valid filter config");

        assert!(!engine.accept(&sample_entry()));
    }

    #[test]
    fn matching_regex_is_accepted() {
        let engine = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: Some(r"/api/\w+".to_string()),
        })
        .expect("valid filter config");

        assert!(engine.accept(&sample_entry()));
    }

    #[test]
    fn non_matching_regex_is_rejected() {
        let engine = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: Some(r"/admin/\w+".to_string()),
        })
        .expect("valid filter config");

        assert!(!engine.accept(&sample_entry()));
    }

    #[test]
    fn invalid_regex_is_rejected_during_construction() {
        let result = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: Some("[".to_string()),
        });

        assert!(result.is_err());
    }

    #[test]
    fn all_filters_can_be_combined() {
        let engine = FilterEngine::new(FilterConfig {
            status: Some(200),
            contains: Some("GET".to_string()),
            regex: Some(r"users".to_string()),
        })
        .expect("valid filter config");

        assert!(engine.accept(&sample_entry()));
    }
}

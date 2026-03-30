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
    config: FilterConfig,
}

#[allow(dead_code)]
impl FilterEngine {
    pub fn new(config: FilterConfig) -> Self {
        Self { config }
    }
    pub fn accept(&self, entry: &LogEntry) -> bool {
        if let Some(status) = self.config.status
            && entry.status != Some(status)
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
        let engine = FilterEngine::new(FilterConfig::default());
        assert!(engine.accept(&sample_entry()));
    }
    #[test]
    fn matching_status_is_accepted() {
        let engine = FilterEngine::new(FilterConfig {
            status: Some(200),
            contains: None,
            regex: None,
        });

        assert!(engine.accept(&sample_entry()));
    }

    #[test]
    fn non_matching_status_is_rejected() {
        let engine = FilterEngine::new(FilterConfig {
            status: Some(404),
            contains: None,
            regex: None,
        });

        assert!(!engine.accept(&sample_entry()));
    }
}

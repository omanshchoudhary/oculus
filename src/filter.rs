use crate::types::LogEntry;
use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use ipnet::IpNet;
use regex::Regex;
use std::net::IpAddr;

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FilterConfig {
    pub status: Option<u16>,
    pub contains: Option<String>,
    pub regex: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub ip: Option<String>,
    pub cidr: Option<String>,
}

#[allow(dead_code)]
pub struct FilterEngine {
    status: Option<u16>,
    contains: Option<String>,
    regex: Option<Regex>,
    from: Option<DateTime<FixedOffset>>,
    to: Option<DateTime<FixedOffset>>,
    ip: Option<IpAddr>,
    cidr: Option<IpNet>,
}

#[allow(dead_code)]
impl FilterEngine {
    pub fn new(config: FilterConfig) -> Result<Self> {
        let compiled_regex = match config.regex {
            Some(pattern) => Some(Regex::new(&pattern)?),
            None => None,
        };
        let from = match config.from {
            Some(value) => Some(DateTime::parse_from_rfc3339(&value)?),
            None => None,
        };
        let to = match config.to {
            Some(value) => Some(DateTime::parse_from_rfc3339(&value)?),
            None => None,
        };
        let ip = match config.ip {
            Some(value) => Some(value.parse::<IpAddr>()?),
            None => None,
        };

        let cidr = match config.cidr {
            Some(value) => Some(value.parse::<IpNet>()?),
            None => None,
        };

        Ok(Self {
            status: config.status,
            contains: config.contains,
            regex: compiled_regex,
            from,
            to,
            ip,
            cidr,
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
        if (self.from.is_some() || self.to.is_some()) && entry.timestamp.is_none() {
            return false;
        }
        if let (Some(from), Some(ts)) = (self.from, entry.timestamp)
            && ts < from
        {
            return false;
        }
        if let (Some(to), Some(ts)) = (self.to, entry.timestamp)
            && ts > to
        {
            return false;
        }

        if self.ip.is_some() || self.cidr.is_some() {
            let entry_ip = match entry.ip.as_deref().and_then(|v| v.parse::<IpAddr>().ok()) {
                Some(ip) => ip,
                None => return false,
            };

            if let Some(ip) = self.ip
                && entry_ip != ip
            {
                return false;
            }

            if let Some(cidr) = self.cidr
                && !cidr.contains(&entry_ip)
            {
                return false;
            }
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
            timestamp: Some(
                DateTime::parse_from_rfc3339("2023-10-10T13:55:36+00:00").expect("valid timestamp"),
            ),
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
            from: None,
            to: None,
            ip: None,
            cidr: None,
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
            from: None,
            to: None,
            ip: None,
            cidr: None,
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
            from: None,
            to: None,
            ip: None,
            cidr: None,
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
            from: None,
            to: None,
            ip: None,
            cidr: None,
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
            from: None,
            to: None,
            ip: None,
            cidr: None,
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
            from: None,
            to: None,
            ip: None,
            cidr: None,
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
            from: None,
            to: None,
            ip: None,
            cidr: None,
        });

        assert!(result.is_err());
    }

    #[test]
    fn all_filters_can_be_combined() {
        let engine = FilterEngine::new(FilterConfig {
            status: Some(200),
            contains: Some("GET".to_string()),
            regex: Some(r"users".to_string()),
            from: None,
            to: None,
            ip: None,
            cidr: None,
        })
        .expect("valid filter config");

        assert!(engine.accept(&sample_entry()));
    }

    #[test]
    fn rejects_invalid_from_timestamp() {
        let result = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: None,
            from: Some("invalid".to_string()),
            to: None,
            ip: None,
            cidr: None,
        });

        assert!(result.is_err());
    }

    #[test]
    fn rejects_invalid_to_timestamp() {
        let result = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: None,
            from: None,
            to: Some("invalid".to_string()),
            ip: None,
            cidr: None,
        });

        assert!(result.is_err());
    }

    #[test]
    fn accepts_entry_within_time_range() {
        let engine = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: None,
            from: Some("2023-10-10T13:00:00+00:00".to_string()),
            to: Some("2023-10-10T14:00:00+00:00".to_string()),
            ip: None,
            cidr: None,
        })
        .expect("valid filter config");

        assert!(engine.accept(&sample_entry()));
    }

    #[test]
    fn rejects_entry_before_from_time() {
        let engine = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: None,
            from: Some("2023-10-10T14:00:00+00:00".to_string()),
            to: None,
            ip: None,
            cidr: None,
        })
        .expect("valid filter config");

        assert!(!engine.accept(&sample_entry()));
    }

    #[test]
    fn rejects_entry_after_to_time() {
        let engine = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: None,
            from: None,
            to: Some("2023-10-10T13:00:00+00:00".to_string()),
            ip: None,
            cidr: None,
        })
        .expect("valid filter config");

        assert!(!engine.accept(&sample_entry()));
    }

    #[test]
    fn exact_ip_filter_accepts_matching_ip() {
        let engine = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: None,
            from: None,
            to: None,
            ip: Some("127.0.0.1".to_string()),
            cidr: None,
        })
        .expect("valid filter config");

        assert!(engine.accept(&sample_entry()));
    }

    #[test]
    fn exact_ip_filter_rejects_non_matching_ip() {
        let engine = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: None,
            from: None,
            to: None,
            ip: Some("10.0.0.1".to_string()),
            cidr: None,
        })
        .expect("valid filter config");

        assert!(!engine.accept(&sample_entry()));
    }

    #[test]
    fn cidr_filter_accepts_matching_subnet() {
        let engine = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: None,
            from: None,
            to: None,
            ip: None,
            cidr: Some("127.0.0.0/24".to_string()),
        })
        .expect("valid filter config");

        assert!(engine.accept(&sample_entry()));
    }

    #[test]
    fn cidr_filter_rejects_non_matching_subnet() {
        let engine = FilterEngine::new(FilterConfig {
            status: None,
            contains: None,
            regex: None,
            from: None,
            to: None,
            ip: None,
            cidr: Some("10.0.0.0/24".to_string()),
        })
        .expect("valid filter config");

        assert!(!engine.accept(&sample_entry()));
    }
}

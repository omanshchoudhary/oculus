use crate::parser::LogParser;
use crate::parser::apache::ApacheParser;
use crate::parser::json::JsonParser;
use crate::parser::nginx::NginxParser;
use crate::types::LogFormat;

#[allow(dead_code)]
pub fn detect_format(lines: &[String]) -> LogFormat {
    let apache = ApacheParser::new();
    let nginx = NginxParser::new();
    let json = JsonParser::new();

    let mut apache_score = 0;
    let mut nginx_score = 0;
    let mut json_score = 0;

    for line in lines {
        if line.trim().is_empty() {
            continue;
        }

        if apache.parse(line).is_ok() {
            apache_score += 1;
        }

        if nginx.parse(line).is_ok() {
            nginx_score += 1;
        }

        if json.parse(line).is_ok() {
            json_score += 1;
        }
    }
    if apache_score == 0 && nginx_score == 0 && json_score == 0 {
        return LogFormat::Apache;
    }

    if apache_score >= nginx_score && apache_score >= json_score {
        LogFormat::Apache
    } else if nginx_score >= json_score {
        LogFormat::Nginx
    } else {
        LogFormat::Json
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::LogFormat;

    #[test]
    fn detect_apache_format() {
        let lines = vec![
            r#"127.0.0.1 - - [10/Oct/2023:13:55:36 +0000] "GET /api/users HTTP/1.1" 200 1234"#
                .to_string(),
            r#"192.168.1.1 - - [10/Oct/2023:13:55:37 +0000] "POST /login HTTP/1.1" 401 456"#
                .to_string(),
        ];

        assert_eq!(detect_format(&lines), LogFormat::Apache);
    }

    #[test]
    fn detect_nginx_format() {
        let lines = vec![
            r#"127.0.0.1 - - [10/Oct/2023:13:55:36 +0000] "GET /health HTTP/1.1" 200 612"#
                .to_string(),
            r#"10.0.0.2 - - [10/Oct/2023:13:55:37 +0000] "POST /login HTTP/1.1" 403 120"#
                .to_string(),
        ];

        assert_eq!(detect_format(&lines), LogFormat::Apache);
    }

    #[test]
    fn detect_json_format() {
        let lines = vec![
            r#"{"ip":"127.0.0.1","method":"GET","path":"/api/users","status":200,"message":"ok"}"#
                .to_string(),
            r#"{"ip":"10.0.0.5","method":"POST","path":"/login","status":401,"message":"denied"}"#
                .to_string(),
        ];

        assert_eq!(detect_format(&lines), LogFormat::Json);
    }

    #[test]
    fn detect_fallback_for_invalid_lines() {
        let lines = vec![
            "not a valid log line".to_string(),
            "still not valid".to_string(),
        ];

        assert_eq!(detect_format(&lines), LogFormat::Apache);
    }

    #[test]
    fn detect_fallback_for_blank_lines() {
        let lines = vec!["".to_string(), "   ".to_string(), "\t".to_string()];

        assert_eq!(detect_format(&lines), LogFormat::Apache);
    }
}

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use oculus::parser::LogParser;
use oculus::parser::apache::ApacheParser;
use oculus::parser::json::JsonParser;
use oculus::parser::nginx::NginxParser;
use oculus::types::{LogEntry, Stats};

fn bench_parse(c: &mut Criterion) {
    let apache_lines = generate_apache_lines(1000);
    let nginx_lines = generate_nginx_lines(1000);
    let json_lines = generate_json_lines(1000);

    let apache_parser = ApacheParser::new();
    let nginx_parser = NginxParser::new();
    let json_parser = JsonParser::new();

    c.bench_function("apache_parse_1000_lines", |b| {
        b.iter(|| {
            for line in &apache_lines {
                let _ = apache_parser.parse(black_box(line));
            }
        })
    });
    c.bench_function("nginx_parse_1000_lines", |b| {
        b.iter(|| {
            for line in &nginx_lines {
                let _ = nginx_parser.parse(black_box(line));
            }
        })
    });
    c.bench_function("json_parse_1000_lines", |b| {
        b.iter(|| {
            for line in &json_lines {
                let _ = json_parser.parse(black_box(line));
            }
        })
    });

    let mut stats = Stats::default();
    let entry = LogEntry {
        ip: Some("127.0.0.1".to_string()),
        method: Some("GET".to_string()),
        path: Some("/api".to_string()),
        status: Some(200),
        timestamp: None,
        message: String::new(),
        raw: r#"127.0.0.1 - - [10/Oct/2000:13:55:36 -0700] "GET /api HTTP/1.1" 200 1234"#
            .to_string(),
    };

    c.bench_function("analyzer_on_line_read", |b| {
        b.iter(|| {
            stats.on_line_read();
        })
    });

    c.bench_function("analyzer_on_parsed_entry", |b| {
        b.iter(|| {
            stats.on_parsed_entry(black_box(&entry));
        })
    });

    c.bench_function("analyzer_on_parse_error", |b| {
        b.iter(|| {
            stats.on_parse_error("ERROR");
        })
    });

    c.bench_function("analyzer_top_paths_sorted", |b| {
        b.iter(|| stats.top_paths_sorted(black_box(10)))
    });
}

criterion_group!(benches, bench_parse);
criterion_main!(benches);

fn generate_apache_lines(n: usize) -> Vec<String> {
    let paths = ["/api", "/health", "/login", "/static/app.js", "/users"];
    let statuses = [200, 200, 200, 404, 500];
    (0..n)
        .map(|i| {
            let path = paths[i % paths.len()];
            let status = statuses[i % statuses.len()];
            format!(
                r#"127.0.0.1 - - [10/Oct/2000:13:55:36 -0700] "GET {path} HTTP/1.1" {status} 1234"#
            )
        })
        .collect()
}

fn generate_nginx_lines(n: usize) -> Vec<String> {
    let paths = ["/api", "/health", "/login", "/static/app.js", "/users"];
    let statuses = [200, 200, 200, 404, 500];
    (0..n)
        .map(|i| {
            let path = paths[i % paths.len()];
            let status = statuses[i % statuses.len()];
            format!(
                r#"127.0.0.1 - - [10/Oct/2000:13:55:36 -0700] "GET {path} HTTP/1.1" {status} 1234"#
            )
        })
        .collect()
}

fn generate_json_lines(n: usize) -> Vec<String> {
    let paths = ["/api", "/health", "/login", "/static/app.js", "/users"];
    let statuses = [200, 200, 200, 404, 500];
    (0..n)
        .map(|i| {
            let path = paths[i % paths.len()];
            let status = statuses[i % statuses.len()];
            format!(
                r#"{{"ip":"127.0.0.1","method":"GET","path":"{path}","status":{status},"timestamp":"2000-10-10T13:55:36Z"}}"#
            )
        })
        .collect()
}

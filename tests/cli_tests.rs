use std::{fs, process::Command};

use tempfile::NamedTempFile;

#[test]
fn test_help() {
    let output = Command::new("cargo")
        .args(["run", "--", "--help"])
        .output()
        .expect("Failed To Execute Command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("oculus"));
    assert!(stdout.contains("Analyze log files"))
}

#[test]
fn test_pipeline_summary_on_apache_fixture() {
    let output = Command::new("cargo")
        .args(["run", "--", "tests/fixtures/apache/access.log"])
        .output()
        .expect("failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("=== Summary ==="));
    assert!(stdout.contains("Total lines: 7"));
    assert!(stdout.contains("Parsed lines: 7"));
    assert!(stdout.contains("Parse errors: 0"));
    assert!(stdout.contains("200 -> 4"));
    assert!(stdout.contains("404 -> 1"));
    assert!(stdout.contains("500 -> 1"));
}
#[test]
fn test_pipeline_malformed_lines_have_line_number_and_no_panic() {
    let output = Command::new("cargo")
        .args(["run", "--", "tests/fixtures/apache/malformed.log"])
        .output()
        .expect("failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stdout.contains("=== Summary ==="));
    assert!(stdout.contains("Total lines: 3"));
    assert!(stdout.contains("Parsed lines: 2"));
    assert!(stdout.contains("Parse errors: 1"));

    assert!(stderr.contains("parse error at line 2"));
}

#[test]
fn test_strict_mode_fails_on_parse_errors() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--fail-on-parse-errors",
            "tests/fixtures/apache/malformed.log",
        ])
        .output()
        .expect("failed to execute command");

    assert!(!output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(stdout.contains("Parse errors: 1"));
    assert!(stderr.contains("encountered 1 parse error(s) with strict mode enabled"));
}

#[test]
fn test_strict_mode_succeeds_when_no_parse_errors_exist() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--fail-on-parse-errors",
            "tests/fixtures/apache/access.log",
        ])
        .output()
        .expect("failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_mixed_filters_status_and_contains() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--status",
            "200",
            "--contains",
            "/api",
            "tests/fixtures/apache/access.log",
        ])
        .output()
        .expect("failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Parsed lines: 2"));
    assert!(stdout.contains("Parse errors: 0"));
    assert!(stdout.contains("200 -> 2"));
}

#[test]
fn test_mixed_filters_status_and_regex() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--status",
            "200",
            "--regex",
            r"/api/\w+",
            "tests/fixtures/apache/access.log",
        ])
        .output()
        .expect("failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Parsed lines: 2"));
    assert!(stdout.contains("200 -> 2"));
}
#[test]
fn test_mixed_filters_no_match() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--status",
            "500",
            "--contains",
            "/dashboard",
            "tests/fixtures/apache/access.log",
        ])
        .output()
        .expect("failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Parsed lines: 0"));
    assert!(stdout.contains("Parse errors: 0"));
}

#[test]
fn test_mixed_filters_status_and_time_range() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--status",
            "200",
            "--from",
            "2023-10-10T13:55:39+00:00",
            "--to",
            "2023-10-10T13:55:41+00:00",
            "tests/fixtures/apache/access.log",
        ])
        .output()
        .expect("failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Parsed lines: 1"));
    assert!(stdout.contains("Parse errors: 0"));
    assert!(stdout.contains("200 -> 1"));
}

#[test]
fn test_mixed_filters_ip_and_regex() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--ip",
            "127.0.0.1",
            "--regex",
            r"/api/\w+",
            "tests/fixtures/apache/access.log",
        ])
        .output()
        .expect("failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Parsed lines: 3"));
    assert!(stdout.contains("200 -> 1"));
    assert!(stdout.contains("404 -> 1"));
    assert!(stdout.contains("500 -> 1"));
}

#[test]
fn test_mixed_filters_cidr_and_contains() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--cidr",
            "192.168.1.0/24",
            "--contains",
            "/api",
            "tests/fixtures/apache/access.log",
        ])
        .output()
        .expect("failed to execute command");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Parsed lines: 1"));
    assert!(stdout.contains("200 -> 1"));
}

#[test]
fn test_output_file_writes_json() {
    let temp = NamedTempFile::new().expect("create temp file");
    let out_path = temp.path().to_path_buf();
    drop(temp); // allow app to create/write file path

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--output",
            "json",
            "--output-file",
            out_path.to_str().expect("utf8 path"),
            "tests/fixtures/apache/access.log",
        ])
        .output()
        .expect("failed to execute command");

    assert!(output.status.success());

    let written = fs::read_to_string(&out_path).expect("read output file");
    assert!(written.contains("\"total_lines\": 7"));
    assert!(written.contains("\"parsed_lines\": 7"));
}

#[test]
fn test_output_file_refuses_overwrite_without_force() {
    let temp = NamedTempFile::new().expect("create temp file");
    fs::write(temp.path(), "existing").expect("write seed content");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--output",
            "csv",
            "--output-file",
            temp.path().to_str().expect("utf8 path"),
            "tests/fixtures/apache/access.log",
        ])
        .output()
        .expect("failed to execute command");

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("already exists"));
    assert!(stderr.contains("--force"));
}

#[test]
fn test_output_file_overwrites_with_force() {
    let temp = NamedTempFile::new().expect("create temp file");
    fs::write(temp.path(), "existing").expect("write seed content");

    let output = Command::new("cargo")
        .args([
            "run",
            "--",
            "--output",
            "csv",
            "--output-file",
            temp.path().to_str().expect("utf8 path"),
            "--force",
            "tests/fixtures/apache/access.log",
        ])
        .output()
        .expect("failed to execute command");

    assert!(output.status.success());

    let written = fs::read_to_string(temp.path()).expect("read output file");
    assert!(written.contains("metric,value"));
    assert!(written.contains("total_lines,7"));
}

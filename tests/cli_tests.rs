use std::process::Command;

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

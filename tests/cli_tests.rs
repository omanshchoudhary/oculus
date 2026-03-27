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

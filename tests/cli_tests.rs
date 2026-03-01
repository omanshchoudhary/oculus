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

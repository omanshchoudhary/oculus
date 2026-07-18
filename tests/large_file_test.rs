use std::fs::File;
use std::io::{BufWriter, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

const LINES: usize = 15_000_000; // ~1.1 GB at ~74 bytes/line

#[test]
#[ignore = "generates a 1GB file; run with: cargo test --release -- --ignored"]
fn large_file_streams_with_flat_memory() {
    // real-disk scratch dir under target/ (auto-deleted when the test ends)
    let dir = tempfile::tempdir_in(env!("CARGO_TARGET_TMPDIR")).expect("create temp dir");
    let log_path = dir.path().join("large.log");

    // generate the synthetic apache log (same shape as the benchmarks)
    let paths = ["/api", "/health", "/login", "/static/app.js", "/users"];
    let statuses = [200, 200, 200, 404, 500];
    let mut writer = BufWriter::new(File::create(&log_path).expect("create log file"));
    for i in 0..LINES {
        writeln!(
            writer,
            r#"127.0.0.1 - - [10/Oct/2000:13:55:36 -0700] "GET {} HTTP/1.1" {} 1234"#,
            paths[i % paths.len()],
            statuses[i % statuses.len()]
        )
        .expect("write line");
    }
    writer.flush().expect("flush");

    let size = std::fs::metadata(&log_path).expect("stat log file").len();
    assert!(
        size >= 1_000_000_000,
        "file is only {size} bytes, expected >= 1GB"
    );

    // run the real binary against it
    let out_path = dir.path().join("report.json");
    let mut child = Command::new(env!("CARGO_BIN_EXE_oculus"))
        .arg(&log_path)
        .args(["--output", "json", "--output-file"])
        .arg(&out_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn oculus");

    // poll peak RSS (VmHWM) from /proc while it runs
    let status_path = format!("/proc/{}/status", child.id());
    let mut peak_kb: u64 = 0;
    loop {
        if let Some(status) = child.try_wait().expect("try_wait") {
            assert!(status.success(), "oculus exited with {status:?}");
            break;
        }
        if let Ok(contents) = std::fs::read_to_string(&status_path)
            && let Some(line) = contents.lines().find(|l| l.starts_with("VmHWM:"))
            && let Some(kb) = line.split_whitespace().nth(1).and_then(|v| v.parse().ok())
        {
            peak_kb = peak_kb.max(kb);
        }
        thread::sleep(Duration::from_millis(50));
    }

    // streaming needs a few MB no matter the file size; slurping the 1GB
    // file would blow far past this bound.
    assert!(peak_kb > 0, "never sampled VmHWM");
    assert!(
        peak_kb < 100 * 1024,
        "peak RSS was {peak_kb} KB, memory grew with file size (streaming broken?)"
    );

    // and the run must be complete + correct
    let report = std::fs::read_to_string(&out_path).expect("read report");
    assert!(report.contains(&format!("\"total_lines\": {LINES}")));
    assert!(report.contains(&format!("\"parsed_lines\": {LINES}")));
}

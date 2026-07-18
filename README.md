# Oculus

```
╔══════════════════════════════════════════════════════════════╗
║                                                              ║
║      ██████╗  ██████╗██╗   ██╗██╗     ██╗   ██╗███████╗      ║
║     ██╔═══██╗██╔════╝██║   ██║██║     ██║   ██║██╔════╝      ║
║     ██║   ██║██║     ██║   ██║██║     ██║   ██║███████╗      ║
║     ██║   ██║██║     ██║   ██║██║     ██║   ██║╚════██║      ║
║     ╚██████╔╝╚██████╗╚██████╔╝███████╗╚██████╔╝███████║      ║
║      ╚═════╝  ╚═════╝ ╚═════╝ ╚══════╝ ╚═════╝ ╚══════╝      ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

A streaming log analyzer in Rust. Parses Apache, Nginx, and JSON logs,
filters them, and exports summaries as table, JSON, or CSV.

**~1.6M lines/sec | ~5 MB memory at any file size | 58 tests | zero-warning CI**

## Features

- **Multi-format parsing**: Apache, Nginx, and JSON log formats, with auto-detection
- **Streaming core**: memory stays flat (~5 MB) whether the file is 1 MB or 100 GB
- **Gzip support**: reads `.gz` compressed logs natively
- **Rich filtering**: status code, substring, regex, time range (`--from`/`--to`), exact IP, and CIDR subnet
- **Three output modes**: colored terminal table, pretty JSON, or CSV (`--output`), to stdout or a file (`--output-file`)
- **Parse-error accounting**: bounded sample of malformed lines with line numbers in every output mode
- **Graceful Ctrl+C**: interrupt a long run and still get a summary of everything processed so far
- **Strict mode**: exit non-zero when any line fails to parse (`--fail-on-parse-errors`)
- **Terminal-aware color**: auto-disables when piped or writing to a file; force off with `--no-color`

## Quickstart

```bash
# Build and test
cargo build
cargo test

# Analyze a log file (format auto-detected)
cargo run -- path/to/access.log
```

### Install (optional)

```bash
# Install locally to run `oculus` directly
cargo install --path .
```

## Usage

Examples use the installed `oculus` binary. From a source checkout, prefix with `cargo run --`.

```bash
# Analyze with auto-detected format
oculus access.log

# Specify a format explicitly
oculus access.log --format apache   # or nginx, json

# Filter by status, substring, or regex
oculus access.log --status 500
oculus access.log --contains "/api/users"
oculus access.log --regex "GET /api/v[0-9]+"

# Filter by time range (RFC 3339)
oculus access.log --from 2023-10-10T00:00:00+00:00 --to 2023-10-11T00:00:00+00:00

# Filter by client IP or subnet
oculus access.log --ip 203.0.113.7
oculus access.log --cidr 10.0.0.0/8

# Export as JSON or CSV
oculus access.log --output json
oculus access.log --output csv --output-file report.csv        # refuses to overwrite
oculus access.log --output csv --output-file report.csv --force

# Analyze a gzip-compressed log
oculus access.log.gz

# Plain output / strict mode / verbose parse errors
oculus access.log --no-color
oculus access.log --fail-on-parse-errors
oculus access.log --verbose
```

Run `oculus --help` for the full flag reference with examples.

## Performance

Measured with Criterion and `cargo flamegraph` on synthetic logs.
Full method and numbers in [`docs/benchmarks.md`](docs/benchmarks.md).

| Parser | Time per line | Throughput |
|--------|---------------|------------|
| Apache | ~0.89 µs | ~1.1M lines/s |
| Nginx  | ~0.90 µs | ~1.1M lines/s |
| JSON   | ~0.37 µs | ~2.7M lines/s |

End-to-end, a 72 MB / 1M-line Apache log completes in ~0.63 s (**~115 MB/s**).
Profiling showed ~25% of runtime went to timestamp parsing that most runs never
use, so it is now parsed lazily, only when a time filter is active (~36% speedup).

Memory is **O(1) in file size**: peak RSS holds at ~5 MB from 740 KB to 216 MB
inputs, enforced by an ignored-by-default 1 GB integration test.

```bash
cargo bench                                               # criterion suite
cargo test --release --test large_file_test -- --ignored  # 1 GB memory guard
```

## Architecture

Oculus is a streaming pipeline. Every line flows through and is dropped;
nothing is accumulated except bounded counters.

```
LogReader -> Parser -> FilterEngine -> Stats -> Report -> Renderer (table/json/csv)
```

| Module   | Path                 | Responsibility                                  |
| -------- | -------------------- | ----------------------------------------------- |
| CLI      | `src/cli.rs`         | Argument parsing via `clap`, `--help` examples  |
| Reader   | `src/reader.rs`      | Streaming file/gzip line reader                 |
| Parser   | `src/parser/`        | Per-format parsers + auto-detector (`LogParser` trait) |
| Filter   | `src/filter.rs`      | Status/substring/regex/time/IP/CIDR predicates  |
| Analyzer | `src/analyzer.rs`    | Counters, status distribution, top-k paths, error samples |
| Report   | `src/output/mod.rs`  | Single serializable summary model               |
| Output   | `src/output/`        | Terminal (color-aware), JSON, and CSV renderers |
| Types    | `src/types.rs`       | `LogEntry`, `LogFormat`, `Stats`                |

The crate is split into a library (`src/lib.rs`, used by benchmarks and tests)
and a thin binary (`src/main.rs`) that wires CLI args into the pipeline.

## Supported Formats

| Format              | Parser                 | Auto-detect |
| ------------------- | ---------------------- | ----------- |
| Apache Combined Log | `src/parser/apache.rs` | ✅          |
| Nginx Access Log    | `src/parser/nginx.rs`  | ~           |
| JSON (structured)   | `src/parser/json.rs`   | ✅          |

Auto-detection scores parsers against the first 50 lines and selects the best match.
Note: in ambiguous tie cases, Nginx-like lines may currently be classified as Apache.
Legend: `~` means partial support or known edge cases.

## Testing

58 tests across unit, integration, snapshot (`insta`), and JSON-schema
assertions, plus regression tests pinning every bug fixed during development.
The 1 GB large-file test is ignored by default and run on demand.

## Development

### Prerequisites

- Latest stable Rust toolchain (edition 2024 support required)

### Quality Gates

All code must pass before merging:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

CI runs these automatically on every push and PR via GitHub Actions.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for coding rules and the PR checklist.

## License

MIT

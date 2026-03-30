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

A fast, reliable Rust CLI for analyzing large log files with low memory usage and actionable summaries.

## Features

- **Multi-format parsing** — Apache, Nginx, and JSON log formats
- **Auto-detection** — Detects log format from file content
- **Streaming processing** — Handles large files with bounded memory via `BufRead`
- **Gzip support** — Natively reads `.gz` compressed log files
- **Flexible filtering** — Filters by status code, substring match, or regex pattern
- **Strict mode** — Exits with an error when parse failures occur (`--fail-on-parse-errors`)
- **Summary output** — Status code distribution, top paths, and parse statistics

## Quickstart

```bash
# 1. Build the project
cargo build

# 2. Run the tests
cargo test

# 3. Analyze a log file (auto-detects format)
cargo run -- path/to/access.log
```

### Install (optional)

```bash
# Install locally to run `oculus` directly
cargo install --path .
```

## Usage

Examples below use the installed `oculus` binary. From a source checkout, run the same commands with `cargo run --` as a prefix.

```bash
# Analyze with auto-detected format
oculus access.log

# Specify a format explicitly
oculus access.log --format apache
oculus app.log --format json
oculus access.log --format nginx

# Filter by HTTP status code
oculus access.log --status 500

# Filter by substring
oculus access.log --contains "/api/users"

# Filter by regex pattern
oculus access.log --regex "GET /api/v[0-9]+"

# Analyze a gzip-compressed log
oculus access.log.gz

# Enable verbose output
oculus access.log --verbose

# Strict mode: exit with error if any line fails to parse
oculus access.log --fail-on-parse-errors
```

## Architecture

Oculus uses a streaming pipeline architecture:

```
Input Reader → Parser → Filter → Analyzer → Output Renderer
```

| Module   | Path              | Responsibility                             |
| -------- | ----------------- | ------------------------------------------ |
| CLI      | `src/cli.rs`      | Argument parsing via `clap`                |
| Reader   | `src/reader.rs`   | Streaming file/gzip reader                 |
| Parser   | `src/parser/`     | Format-specific parsers and auto-detector  |
| Filter   | `src/filter.rs`   | Status, substring, and regex filtering     |
| Analyzer | `src/analyzer.rs` | Counters, status distribution, top-k paths |
| Output   | `src/output/`     | Terminal summary renderer                  |
| Types    | `src/types.rs`    | `LogEntry`, `LogFormat`, `Stats`           |
| Errors   | `src/error.rs`    | Typed errors (`thiserror`) and app-level `anyhow` handling |

## Supported Formats

| Format              | Parser                 | Auto-detect |
| ------------------- | ---------------------- | ----------- |
| Apache Combined Log | `src/parser/apache.rs` | ✅          |
| Nginx Access Log    | `src/parser/nginx.rs`  | ~           |
| JSON (structured)   | `src/parser/json.rs`   | ✅          |

Auto-detection scores parsers against the first 50 lines and selects the best match.
Note: in ambiguous tie cases, Nginx-like lines may currently be classified as Apache.
Legend: `~` means partial support or known edge cases.

## Development

### Prerequisites

- Latest stable Rust toolchain (edition 2024 support required)

### Quality Gates

All code must pass before merging:

```bash
# Format
cargo fmt

# Lint (warnings = errors)
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test --all-features
```

CI runs these automatically on every push and PR via GitHub Actions.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for coding rules and the PR checklist.

## License

MIT

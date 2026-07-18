# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-07-18

First stable release.

### Added

- Streaming pipeline that processes logs line by line with flat memory
  (~5 MB peak regardless of file size)
- Parsers for Apache combined, Nginx access, and structured JSON logs,
  with auto-detection over the first 50 lines
- Gzip support: `.gz` logs are read natively
- Filters: status code, substring, regex, RFC 3339 time range
  (`--from`/`--to`), exact IP, and CIDR subnet, freely combinable
- Output modes: colored terminal table, pretty JSON, and CSV via `--output`,
  written to stdout or a file with `--output-file` (overwrite requires `--force`)
- Parse-error accounting: bounded sample of malformed lines with line
  numbers, surfaced in all three output modes
- Strict mode (`--fail-on-parse-errors`) for CI-style non-zero exits
- Graceful Ctrl+C handling: interrupting a run still flushes a summary of
  all lines processed so far
- Terminal-aware color with `--no-color` override; color auto-disables when
  piped or writing to a file
- `--help` usage examples and `--verbose` diagnostics
- Criterion benchmark suite (parsers and analyzer) with synthetic data
  generators, documented in `docs/benchmarks.md`
- Ignored-by-default 1 GB integration test asserting peak memory stays flat
- Regression tests pinning every bug fixed during development
- GitHub Actions CI: fmt, clippy (warnings deny), tests, release build

### Performance

- Profiled with `cargo flamegraph`; timestamp parsing (~25% of runtime) is
  now lazy and only runs when a time filter is active, a ~36% speedup on the
  common path
- Throughput: ~1.6M lines/sec end to end on synthetic Apache logs (~115 MB/s)

[1.0.0]: https://github.com/omanshchoudhary/oculus/releases/tag/v1.0.0

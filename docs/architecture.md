# Architecture

Oculus is a single-pass streaming pipeline. Every log line is read, parsed,
filtered, folded into bounded aggregates, and dropped. No stage holds more
than one line at a time, which keeps peak memory at ~5 MB regardless of
input size (see ADR 001 and `benchmarks.md`).

## Data flow

```
file / .gz
   |
LogReader            src/reader.rs      yields (line_no, String), buffered
   |
LogParser            src/parser/        line -> Result<LogEntry, String>
   |                                    (apache | nginx | json, auto-detected)
FilterEngine         src/filter.rs      accept(&LogEntry) -> bool
   |
Stats                src/analyzer.rs    fold: counters, status map,
   |                                    path map, error samples (max 5)
Report               src/output/mod.rs  sorted, serializable snapshot
   |
Renderer             src/output/        table (color-aware) | json | csv
```

## Key types and contracts

- `LogParser` (trait): one implementation per format. Parsers are stateless
  after construction; `parse` borrows the line and allocates only for the
  fields it extracts.
- `LogEntry`: the normalized record. All fields except `raw` and `message`
  are `Option`, because real logs are ragged. `timestamp` is populated only
  when a time filter needs it (see ADR 002).
- `Stats`: the only mutable state in a run. Updated via three methods
  (`on_line_read`, `on_parsed_entry`, `on_parse_error`) so the invariants
  live in one file.
- `Report`: built once from `Stats` after the stream ends. Sorting happens
  here (status codes ascending, paths by count) so all three renderers emit
  deterministic output from the same source of truth. `Report` derives
  `Serialize`; the JSON renderer is one `serde_json` call.

## Crate layout

The crate is a library (`src/lib.rs`) plus a thin binary (`src/main.rs`).
The binary owns everything user-facing: CLI parsing, format auto-detection,
the read loop, Ctrl+C handling, and the color decision. The library owns the
pipeline stages, which is what allows Criterion benches and integration
tests to drive them directly as `oculus::...`.

Errors follow the standard split: recoverable per-line parse failures are
`Result<_, String>` values counted in `Stats`, while fatal conditions (bad
CLI input, unreadable file, refusing to overwrite output) bubble up through
`anyhow::Result` in the binary.

## Reliability behavior

- Ctrl+C: a signal handler flips an `Arc<AtomicBool>`; the read loop checks
  it each iteration and breaks, and the normal summary path then renders
  whatever was processed. Partial aggregates are always internally
  consistent.
- Strict mode (`--fail-on-parse-errors`) exits non-zero after rendering, so
  the summary is available even when the run fails.
- The 1 GB integration test (`tests/large_file_test.rs`, ignored by default)
  asserts peak RSS stays under 100 MB, guarding the streaming property
  against regressions.

## Decision records

- [ADR 001: Streaming pipeline over in-memory loading](adr/001-streaming-pipeline.md)
- [ADR 002: Lazy timestamp parsing driven by profiling](adr/002-lazy-timestamp-parsing.md)

Performance method and measured numbers: [benchmarks.md](benchmarks.md).

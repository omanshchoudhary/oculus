# ADR 001: Streaming pipeline over in-memory loading

Date: 2026-07-19
Status: Accepted

## Context

Log files routinely exceed available RAM (multi-GB access logs are normal in
production). The project's core non-functional requirement is that memory must
not grow with file size. The obvious implementation, `fs::read_to_string`
followed by iteration, holds the entire file in memory and fails exactly when
the tool is most needed.

## Decision

Process logs as a stream: a `BufRead`-based `LogReader` yields one line at a
time, each line flows through parser, filter, and analyzer, and is then
dropped. The only state kept across lines is bounded aggregates in `Stats`
(counters, a status-code map, a path-count map, and at most five parse-error
samples). The final `Report` is built once, after the stream ends.

## Alternatives considered

- **Load whole file (`read_to_string`)**: simplest code, but O(file size)
  memory. Rejected as it violates the core requirement.
- **Memory-mapped file (`mmap`)**: fast random access, but adds an unsafe
  dependency, complicates gzip support (a stream, not a file), and still
  pressures the page cache on huge files. Random access is not needed for a
  single-pass summary.
- **Chunked parallel processing**: higher throughput ceiling, but adds
  ordering and aggregation complexity. Deferred until profiling shows the
  single-threaded pipeline is a bottleneck (it currently processes ~115 MB/s).

## Consequences

- Peak RSS holds at ~5 MB from 740 KB to 216 MB inputs (measured), enforced
  by a 1 GB integration test asserting peak RSS stays under 100 MB.
- Gzip support falls out naturally, since the reader wraps any byte stream.
- Graceful Ctrl+C was cheap to add: partial aggregates are always valid.
- The trade-off: single pass only. Features needing random access or a second
  pass (percentiles over raw values, sorting raw lines) would require a
  different design or an explicit re-read.
- Memory scales with the number of distinct paths in the path map, not with
  line count. Pathological inputs (millions of unique URLs) would grow it;
  acceptable for v1 and documented in `docs/benchmarks.md`.

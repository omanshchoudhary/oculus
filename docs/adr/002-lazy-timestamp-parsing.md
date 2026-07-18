# ADR 002: Lazy timestamp parsing driven by profiling

Date: 2026-07-19
Status: Accepted

## Context

CPU profiling with `cargo flamegraph` over a 1M-line Apache log showed ~25% of
total runtime inside chrono timestamp parsing (`strftime` iteration plus
`parse_internal`). Auditing consumers showed the parsed timestamp is read in
exactly one place: the `--from`/`--to` time filters. The analyzer and all
three output renderers never touch it. On runs without a time filter, a
quarter of the work produced a value that was thrown away.

## Decision

Make timestamp parsing opt-in at parser construction:
`ApacheParser::new().with_timestamps(enabled)`. The binary enables it only
when `--from` or `--to` is present. When disabled, `LogEntry.timestamp` is
`None` and the chrono call never runs. A builder method was chosen over
changing `new()` so existing callers (tests, benches, `Default`) stay valid.

## Alternatives considered

- **Always parse (status quo)**: simplest, but pays ~25% runtime for an
  unused value on the common path.
- **Faster hand-written timestamp parser**: keeps eager semantics and would
  help the filtered path too, but is more code to get right for a cost that
  is zero when skipped entirely. Can still be done later for the filtered
  path if profiling justifies it.
- **Per-entry lazy cell (parse on first access)**: transparent to consumers,
  but adds interior mutability and lifetime complexity to `LogEntry` for no
  practical gain over a constructor flag in a pipeline this small.

## Consequences

- Common-path runtime on a 72 MB log dropped from ~0.99 s to ~0.63 s
  (~36% faster, measured on the release binary; see `docs/benchmarks.md`).
- Filtered runs behave exactly as before; the time-range integration test
  pins this (`test_mixed_filters_status_and_time_range`).
- The coupling is now implicit: any future feature that reads timestamps on
  every line (for example, requests-per-hour analytics) must enable
  timestamps for that code path. This is documented at the switch in
  `src/parser/apache.rs` and in `ApacheParser::with_timestamps`.

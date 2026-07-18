# Benchmarks

Baseline performance numbers for the hot paths in oculus, measured with
[Criterion](https://github.com/bheisler/criterion.rs).

## How to reproduce

```bash
cargo bench
```

Criterion reports the median time per iteration along with a confidence
interval. Full HTML reports (with plots) are written to
`target/criterion/`. To run a single group, filter by name:

```bash
cargo bench -- analyzer   # only the analyzer benchmarks
cargo bench -- parse      # only the parser benchmarks
```

## Environment

| | |
|---|---|
| CPU | Intel Core i7-10610U @ 1.80 GHz (8 threads) |
| OS | Linux 6.17.0-35-generic |
| Toolchain | rustc 1.96.0 |
| Profile | `bench` (optimized) |

Numbers are single-threaded. Absolute values depend on hardware; treat them as
a baseline to catch regressions, not as a hardware spec.

## Parser throughput

Each parser is measured against a synthetic batch of 1000 lines generated in
the benchmark (deterministic: cycling paths and status codes). The per-line
figure is the batch time divided by 1000.

| Parser | 1000 lines | Per line |
|--------|------------|----------|
| Apache | ~889 µs | ~0.89 µs |
| Nginx  | ~904 µs | ~0.90 µs |
| JSON   | ~372 µs | ~0.37 µs |

The regex-based parsers (Apache, Nginx) are ~2.4x slower than the JSON parser,
which is expected, since regex matching dominates their cost.

## Analyzer update functions (per call)

Measured against a single pre-built `LogEntry` / `Stats`.

| Operation | Median time / call |
|-----------|--------------------|
| `on_line_read`      | ~38 ps  |
| `on_parse_error`    | ~3.2 ns |
| `top_paths_sorted`  | ~61 ns  |
| `on_parsed_entry`   | ~72 ns  |

`on_line_read` is a single counter increment, so it is effectively free.
`top_paths_sorted` here sorts a single-entry map; the cost grows with the
number of distinct paths, so this is a floor rather than a realistic load.

## CPU profiling

Profiled with `cargo flamegraph` against a 1,000,000-line synthetic Apache log
(`cargo flamegraph --bin oculus -- <big.log> --output json --output-file /dev/null`).
Samples grouped by what the work actually does:

| Cost area | ~% CPU | What it is |
|-----------|--------|------------|
| Timestamp parsing (chrono) | ~25% | decoding the `[.. timestamp ..]` field on every line |
| Regex matching | ~28% | DFA search (~23%) + capture lookup by name (~5%) |
| Allocations (malloc/free) | ~15% | one `String` per field per line |
| HashMap hashing | ~6% | `status_counts` / `top_paths` updates |

Key finding: timestamp parsing is ~25% of runtime, but the timestamp is only
needed when a `--from`/`--to` time filter is active. On the common path it is
wasted work. The largest win is to parse it lazily rather than to make parsing
faster. Secondary wins: look up regex capture groups by index instead of by
name (~5%), and trim per-field `String` allocations (~15%).

## Optimizations applied

### Lazy timestamp parsing

The Apache parser now skips timestamp decoding unless a time filter needs it
(`ApacheParser::with_timestamps`, wired from `--from`/`--to` in `main`).

Measured on the full binary over a 1,000,000-line / 72 MB Apache log, no time
filter (the common path):

| | Runtime | Peak memory |
|---|---------|-------------|
| Before | ~0.99 s | ~4.8 MB |
| After  | ~0.63 s | ~4.8 MB |

~36% faster on the common path; memory is unchanged and stays flat regardless
of file size (streaming). When a time filter is active the timestamp is parsed
as before, so filtering results are unchanged.

## Memory behavior

Peak RSS (`/usr/bin/time -f %M`) across a 290x range of input sizes:

| File | Lines | Size | Peak memory | Runtime |
|------|-------|------|-------------|---------|
| tiny  | 10 K | 740 KB | ~4.7 MB | 0.00 s |
| large | 1 M  | 72 MB  | ~4.9 MB | 0.66 s |
| huge  | 3 M  | 216 MB | ~4.8 MB | 2.02 s |

Memory is O(1) in file size while runtime is O(n) in line count, the
signature of a streaming design: every line is visited, none are kept.
Memory scales only with the number of *distinct* paths/statuses held in the
stats maps, not with total lines.

This is enforced by an ignored-by-default integration test that generates a
>= 1 GB log, runs the binary against it, and asserts peak RSS stays under
100 MB (`tests/large_file_test.rs`):

```bash
cargo test --release --test large_file_test -- --ignored
```

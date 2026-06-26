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
which is expected — regex matching dominates their cost.

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

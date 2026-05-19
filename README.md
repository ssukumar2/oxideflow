# oxideflow

A log file analyzer CLI written in Rust. Filters log lines by level and regex, produces summary statistics, detects anomalies, and exports results in multiple formats.

## Features

- Filter by log level (`ERROR`, `WARN`, `INFO`, `DEBUG`, `TRACE`)
- Regex pattern matching with grep-style `before`/`after` context
- Time-range filtering by timestamp prefix
- Deduplicate repeated messages with configurable window
- Follow mode (`tail -f` style live streaming)
- TOML-based configuration
- Multi-file merge sorted by timestamp
- Trigram-based message clustering for similar errors
- Stack trace detection and grouping
- ASCII histogram rendering for level distributions
- Error spike detection with hourly bucket analysis
- IPv4, UUID, and HTTP status code extraction
- Token, email, and PII redaction
- Word frequency analysis
- Streaming reader for huge files

## Output formats

- Plain text with ANSI colors
- JSON (pretty)
- NDJSON (one object per line, streaming-friendly)
- CSV
- Prometheus text format for metrics scraping

## Build

```bash
cargo build --release
```

Binary lands at `target/release/oxideflow`.

## Usage

```bash
oxideflow filter examples/sample.log --level ERROR
oxideflow filter examples/sample.log --pattern "timeout|refused"
oxideflow filter examples/sample.log --since "2026-04-16 10:00"
oxideflow filter examples/sample.log --json
```

## Modules

| Module      | Responsibility                                      |
|-------------|-----------------------------------------------------|
| `parser`    | Read files, detect levels, normalize, slice         |
| `filter`    | Level, regex, time, IP, status code, UUID filters   |
| `stats`     | Counts, error rate, throughput, percentages         |
| `summary`   | Aggregate Report struct combining all metrics       |
| `dedup`     | Suppress repeats, rank top errors                   |
| `timefilter`| Parse timestamps, compute gaps                      |
| `follow`    | Tail-style live streaming                           |
| `histogram` | ASCII bar chart renderer                            |
| `merge`     | Combine multiple log files by timestamp             |
| `search`    | Pattern search with context lines                   |
| `anomaly`   | Error spike detection by hourly buckets             |
| `redact`    | Scrub emails, tokens, and PII                       |
| `tokenize`  | Word frequency and top-word analysis                |
| `cluster`   | Trigram similarity clustering                       |
| `diff`      | Compare level counts across two log slices          |
| `metrics`   | Prometheus text-format exporter                     |
| `output`    | JSON, NDJSON, CSV, colored printers                 |
| `config`    | TOML configuration loader                           |
| `error`     | Project-wide `OxideError` via `thiserror`           |

## Stack

Rust 2021, clap 4.5, regex, serde/serde_json, anyhow, thiserror, colored, toml 0.8.

## Tests

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
```

## CI

GitHub Actions on every push to `main` and `dev`: format check, clippy with `-D warnings`, build, test, cargo audit.

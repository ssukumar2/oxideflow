# oxideflow

A log file analyzer CLI written in Rust. Filters log lines by level and regex, and produces summary statistics. Designed as a foundation for larger log processing tools.

## Why this project

Log files are everywhere in systems work, but most tools are either very simple (grep) or very heavy (full log management platforms). oxideflow sits in the middle — a fast, portable CLI that does filtering and basic analysis with clean structured output. Built to explore Rust's strengths in text processing, error handling, and CLI ergonomics.

## Features

- Filter by log level (INFO, WARN, ERROR, DEBUG, TRACE)
- Filter by regex pattern
- Combine level and pattern filters
- Plain text or JSON output
- Summary statistics (counts by level)
- Unit tests for parser and filter logic

## Usage

Build:

    cargo build --release

Filter by level:

    cargo run -- filter --file examples/sample.log --level ERROR

Filter by pattern:

    cargo run -- filter --file examples/sample.log --pattern "connection"

Combine filters with JSON output:

    cargo run -- filter --file examples/sample.log --level INFO --pattern "user" --json

Print summary stats:

    cargo run -- stats --file examples/sample.log

## Roadmap

- Support for structured log formats (JSON logs, logfmt)
- Time range filtering (--since, --until)
- Follow mode for live tailing
- Colored output for terminals
- Parallel processing for very large files

## Building and testing

Requires Rust 1.70+.

    cargo build
    cargo test

## License

MIT
# oxideflow

A log file analyzer CLI written in Rust that filters log files by level, regex pattern, or time range, computes summary statistics, detects error spikes, deduplicates repeated messages, groups stack traces, follows files live like tail -f, and outputs results as plain text, JSON, NDJSON, CSV, or Prometheus format. Build with `cargo build --release`, run with `oxideflow filter examples/sample.log --level ERROR`, and test with `cargo test`.


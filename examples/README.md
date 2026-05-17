# Examples

Sample log files for testing oxideflow.

## Files
- `sample.log` — small hand-written sample covering INFO/WARN/ERROR levels.
- `big_sample.log` — larger generated sample for performance testing.
- `generate_big_log.sh` — script that produces `big_sample.log`.

## Generating a fresh big sample
```bash
cd examples
./generate_big_log.sh
```

## Usage with oxideflow
```bash
cargo run -- filter examples/sample.log --level ERROR
cargo run -- filter examples/sample.log --pattern "timeout"
```

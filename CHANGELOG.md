# Changelog

All notable changes to oxideflow are documented here.

## [Unreleased]

### Added
- `stats::count_levels` and `stats::total_lines` helpers.
- `filter::filter_by_level` and `filter::errors_only` helpers.
- `output::to_json` JSON serializer for log lines.
- `output::print_colored` colored terminal printer.
- `dedup::top_errors` aggregator for most-frequent error messages.
- `config::default_levels` and `config::is_known_level` helpers.
- Unit tests for stats, filter, and dedup modules.

## [0.1.0]

### Added
- Initial CLI scaffold using clap 4.5.
- Modules: parser, filter, output, stats, timefilter, follow, dedup, config.

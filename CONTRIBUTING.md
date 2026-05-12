# Contributing to oxideflow

Thanks for your interest in contributing!

## Branching
- `main` — stable, release-ready code.
- `dev`  — daily integration branch. Open PRs from feature branches into `dev`,
  then merge `dev` into `main` for releases.

## Local setup
```bash
git clone https://github.com/ssukumar2/oxideflow.git
cd oxideflow
cargo build
cargo test
```

## Commit style
- Use Conventional Commits: `feat:`, `fix:`, `docs:`, `test:`, `refactor:`, `chore:`.
- Keep commits small and focused on a single change.

## Code conventions
- The primary line type is `LogLine` (defined in `src/parser.rs`).
- Raw text is stored in the `raw` field, not `message`.
- `level` is `Option<String>` — handle `None` explicitly.
- Run `cargo fmt` and `cargo clippy` before opening a PR.

## Tests
Add unit tests in the same file under `#[cfg(test)] mod tests`.

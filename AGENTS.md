# AGENTS.md

## Project Overview

`rpos` is a Rust library providing cursor position management on a 2D table with bounds checking.

## Build & Test Commands

```bash
cargo build          # Build the project
cargo test           # Run all tests
cargo test <name>    # Run specific test
cargo fmt            # Format code
cargo clippy         # Run linter
cargo doc --open     # Generate and view documentation
```

## Code Style

- Use `anyhow` for error handling with `Result<T>` and `anyhow::bail!` for errors
- Follow Rust 2021 edition conventions
- Use `0-indexed` positions for cursor (line, column)
- Keep modules focused: `cursor.rs` for Cursor logic, `table.rs` for Table logic
- Prefer explicit validation over silent failures

## Architecture

```
src/
  lib.rs       # Module exports
  cursor.rs    # Cursor struct with movement and validation
  table.rs     # Table struct wrapping Cursor with dimensions
tests/
  cursor_test.rs
  table_test.rs
```

## Testing Guidelines

- Write tests in `tests/` directory for integration tests
- Cover boundary conditions (0, max-1 positions)
- Test error cases with invalid inputs

## Release Process

- Uses release-please for automated releases
- Changelog is auto-generated from conventional commits
- Publish to crates.io via GitHub Actions

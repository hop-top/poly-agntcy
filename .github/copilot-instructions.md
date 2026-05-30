# Copilot Instructions

## Project

- Name: poly-agntcy
- Description: Polyglot SDK suite for the AGNTCY Agent Directory Service (DIR)
- License: Apache-2.0

## Languages

- Go: use standard library where possible, `golangci-lint` for linting
- Rust: stable toolchain, `cargo fmt` + `cargo clippy -D warnings`, `cargo test --all-features` for testing
- TypeScript: strict mode, ESLint + Prettier, vitest for testing
- Python: type hints required, ruff for linting, pytest for testing

## Build

- Build system: Make
- Run `make check` before committing
- Commits: Conventional Commits (feat|fix|refactor|...)

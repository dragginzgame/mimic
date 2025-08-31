# Repository Guidelines

## Project Structure & Module Organization
- `crates/mimic`: Core framework (entities, queries, macros, types).
- `crates/{mimic_build,mimic_common,mimic_schema}`: Codegen and shared utilities.
- `crates/mimic_tests/{canister,design,src}`: Integration and design tests.
- `assets/`: Images and docs assets. `scripts/`: release/version helpers. `Makefile`: common tasks.
- Workspace manifest: `Cargo.toml` (edition 2024, rust-version 1.89.0).

## Build, Test, and Development Commands
- `make check`: Fast type-check for all crates.
- `make test`: Run all unit/integration tests (`cargo test --workspace`).
- `make build`: Release build for the workspace.
- `make clippy`: Lints with warnings denied.
- `make fmt` / `make fmt-check`: Format or verify formatting.
- Versioning: `make version|tags|patch|minor|major|release` (see `VERSIONING.md`).

## Coding Style & Naming Conventions
- Rustfmt: 4-space indent, edition 2024; run `cargo fmt --all` before committing.
- Naming: `snake_case` for modules/functions/files, `CamelCase` types/traits, `SCREAMING_SNAKE_CASE` consts.
- Linting: Code must pass `make clippy`; prefer `?` over `unwrap()`, handle errors explicitly.
- Keep public APIs documented; co-locate small unit tests in the same file under `mod tests`.

## Testing Guidelines
- Framework: Rust test harness. Place unit tests near code; integration tests live in `crates/mimic_tests`.
- Run all: `make test`. Single crate/test: `cargo test -p mimic <filter>`.
- Aim for meaningful coverage on entity macros, query paths, and index behavior. Add fixtures where helpful.

## Commit & Pull Request Guidelines
- Commits: Imperative mood, concise scope (e.g., "Fix index serialization"; "Bump version to 0.15.1").
- PRs: Clear description, linked issues, rationale, before/after notes; include tests and docs updates.
- Changelog: Update `CHANGELOG.md` under `[Unreleased]` for user-visible changes.
- Releases: Use `make patch|minor|major` and push with tags; do not hand-edit tags.

## Security & Configuration Tips
- Tag immutability: run `make security-check`; never modify pushed release tags.
- Pin git dependencies by tag in downstream projects.
- Toolchain: install Rust `1.89.0` (`rustup toolchain install 1.89.0`) and ensure CI matches.


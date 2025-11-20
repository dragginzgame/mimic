
![MSRV](https://img.shields.io/badge/rustc-1.91+-blue.svg)
[![CI](https://github.com/dragginzgame/mimic/actions/workflows/ci.yml/badge.svg)](https://github.com/dragginzgame/mimic/actions/workflows/ci.yml)

# Mimic — Data Model Framework for the Internet Computer

![Funny / appealing cover image for Mimic](assets/image.png)<img src="assets/swampfree.png" alt="100% Certified Swamp-Free" width="200"/>

> Battle-tested, schema-first data models for Internet Computer canisters. Built for [Dragginz](https://dragginz.io/), now open to everyone.

```
Make It [ Matter     ] on the Internet Computer
          Magical
          Modular
          Multiplayer
          Monetisable
          Mainstream
```

## 👋 Overview

**Mimic** is a Rust framework for building strongly-typed, queryable data models on the [Internet Computer](https://internetcomputer.org).

---

## ✨ Highlights

- **Entity macros** – define entities declaratively with schema attributes.
- **Query builder** – type-safe filters, sorting, offsets, limits.
- **Stable storage** – powered by `ic-stable-structures` B-Trees with predictable costs.
- **Automatic endpoints** – `mimic_build` generates `mimic_query_load`, `mimic_query_save`, `mimic_query_delete` handlers.
- **Observability endpoints** – `mimic_snapshot`, `mimic_logs`, `mimic_metrics`, `mimic_metrics_reset` ship automatically.
- **Text casing toolkit** – sanitizers/validators for snake/kebab/title/camel cases that work across lists, maps, sets.
- **Integration with IC canisters** – ergonomic `mimic_start!` and `mimic_build!` macros.
- **Testability** – fixtures, query validation, index testing utilities.

---

## ⚡ Quickstart

1. **Install Rust 1.91.1+** (workspace uses edition 2024).
2. **Add Mimic** to your `Cargo.toml` using the latest tag:
   ```toml
   [dependencies]
   mimic = { git = "https://github.com/dragginzgame/mimic.git", tag = "v0.29.0" }
   ```
3. **Declare an entity** with the `#[entity]` macro and a primary key.
4. **Query your data** via `db!().load::<Entity>()...`.

See [INTEGRATION.md](INTEGRATION.md) for pinning strategies, feature flags, and troubleshooting tips.

---

## 🚀 Example

### Define an entity

```rust
/// Rarity
/// Affects the chance of an item dropping or an event occurring.
#[entity(
    sk(field = "id"),
    fields(
        field(ident = "id", value(item(is = "types::Ulid"))),
        field(ident = "name", value(item(is = "text::Name"))),
        field(ident = "description", value(item(is = "text::Description"))),
        field(ident = "order", value(item(is = "game::Order"))),
        field(ident = "color", value(item(is = "types::color::RgbHex"))),
    ),
)]
pub struct Rarity {}
```

### Query entities

```rust
#[query]
pub fn rarities() -> Result<Vec<RarityView>, mimic::Error> {
    let query = mimic::db::query::load()
        .filter(|f| {
            // (level >= 2 AND level <= 4) OR (name CONTAINS "ncon")
            (f.gte("level", 2) & f.lte("level", 4)) | f.contains("name", "ncon")
        })
        .sort(|s| s.desc("level"))
        .limit(100);

    let rows = db().load::<Rarity>().debug().execute(&query)?;
    Ok(rows.views())
}
```

---

## 🏗️ Project Layout

- `crates/mimic` — core framework (entities, queries, schema, stores, types).
- `crates/mimic_build` — canister codegen (`build.rs` → `actor.rs`).
- `crates/mimic_common` — shared utilities.
- `crates/mimic_schema` — schema definitions and types.
- `crates/mimic_declare` — proc-macros for schema/traits.
- `crates/mimic_tests` — integration + design tests.
- `assets/` — artwork and documentation assets. `scripts/` — release/version helpers.

---

## 📟 Observability & Tooling

- `mimic_snapshot()` → live `StorageReport` with data/index/state breakdowns.
- `mimic_logs()` → in-memory log buffer (oldest → newest).
- `mimic_metrics()` → `EventReport` for counters since `since_ms`.
- `mimic_metrics_reset()` → clears metrics state.

Examples:
```bash
dfx canister call <canister> mimic_snapshot
dfx canister call <canister> mimic_logs
dfx canister call <canister> mimic_metrics
dfx canister call <canister> mimic_metrics_reset
```

---

## 🧑‍💻 Local Development

Workspace commands (see `Makefile`):

```bash
make check      # type-check workspace
make clippy     # lint with warnings denied
make test       # run all unit + integration tests
make fmt        # format the workspace (or fmt-check to verify)
make build      # release build
```

Pre-commit hooks run `cargo fmt -- --check`, `cargo sort --check`, and `cargo sort-derives --check`. Run any of the `make fmt*`, `make clippy`, or `make check` targets once to auto-install and enable them.

### Style & conventions

- Prefer `?` + typed errors (`thiserror`) instead of panics in library code.
- Keep functions focused; extract helpers when logic grows.
- Import ergonomically: group paths per crate (e.g., `use crate::{db, design};`).
- Use saturating arithmetic for counters and totals.
- Co-locate small unit tests; integration/design tests live in `crates/mimic_tests`.
- No backward-compatibility promise yet—document breaking changes in the changelog.

---

## 🤝 Contributing & Support

We welcome issues, discussions, and pull requests now that the repository is public. To contribute:

1. Fork and clone the repo.
2. Install the toolchain (`rustup toolchain install 1.91.1`).
3. Run `make fmt-check && make clippy && make check && make test` before opening a PR.
4. Document user-visible changes in [CHANGELOG.md](CHANGELOG.md) under the latest heading.

Need help? Start with [INTEGRATION.md](INTEGRATION.md), [VERSIONING.md](VERSIONING.md), or open a GitHub issue.

---

## 📊 Current Focus

- Expanding documentation and runnable examples.
- Improving error modeling (`MimicError` + nested domain errors).
- Deepening test coverage across entity indexes and query paths.
- Tracking store statistics & memory usage in production deployments.
- Reducing WASM size produced by `mimic_build`.

---

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 (`LICENSE-APACHE`)
- MIT license (`LICENSE-MIT`)

at your option.

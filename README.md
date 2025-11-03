

![MSRV](https://img.shields.io/badge/rustc-1.91+-blue.svg)
[![CI](https://github.com/dragginzgame/mimic/actions/workflows/ci.yml/badge.svg)](https://github.com/dragginzgame/mimic/actions/workflows/ci.yml)

# Mimic — Data Model Framework for the Internet Computer

![Funny / appealing cover image for Mimic](assets/image.png)<img src="assets/swampfree.png" alt="100% Certified Swamp-Free" width="200"/>

```

Make It [ Matter     ] on the Internet Computer
          Magical
          Modular
          Multiplayer
          Monetisable
          Mainstream

````


## 👋 Introduction

**Mimic** is a Rust framework for building strongly-typed, queryable data models on the [Internet Computer](https://internetcomputer.org).

It was originally built for the Web3 game [Dragginz](https://dragginz.io/). Mimic aims to make building **schemas, queries, and storage-backed entities** ergonomic, safe, and fun.

---

## ⚡ Quickstart

1) Add dependency (pin to a release tag): `mimic = { git = "git@github.com:dragginzgame/mimic.git", tag = "v0.21.0" }` (or HTTPS with token)
2) Define an entity with `#[entity]` and a primary key.
3) Query with `db().load::<Entity>().filter(|f| ...).sort(|s| ...).limit(...).execute()?`.

---

## ✨ Features

- **Entity macros** — define entities declaratively with schema attributes
- **Query builder** — type-safe filters, sorting, offsets, limits
- **Stable storage** — powered by `ic-stable-structures` B-Trees
- **Automatic endpoints** — `mimic_build` generates `mimic_query_load`, `mimic_query_save`, `mimic_query_delete`
- **Observability endpoints** — `mimic_snapshot`, `mimic_logs`, `mimic_metrics`, `mimic_metrics_reset`
- **Text casing toolkit** — sanitizers and validators for lower/upper/snake/kebab/title/camel cases (works across lists, maps, sets)
- **Integration with IC canisters** — ergonomic `mimic_start!` and `mimic_build!` macros
- **Testability** — fixtures, query validation, and index testing

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
````

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

## 📦 Crates

- `crates/mimic` — core framework (entities, queries, schema, stores, types)
- `crates/mimic_build` — canister codegen (`build.rs` → `actor.rs`)
- `crates/mimic_common` — shared utilities
- `crates/mimic_schema` — schema definitions and types
- `crates/mimic_declare` — proc-macros for schema/traits
- `crates/mimic_tests` — integration/design tests

---

## 🔧 Modules (in `mimic`)

- `core` — traits, keys, type system, validation.
- `db` — query execution, stores, registries, persistence.
- `design` — schema macros and design-time structures.
- `interface` — canister call helpers for query endpoints.
- `macros` — crate macros (`mimic_start!`, `mimic_build!`, `db!`).
- `obs` — observability: metrics, logs, and storage snapshots.

---

## 🧑‍💻 Development

### Install

See [INTEGRATION.md](INTEGRATION.md) for installation and integration details.

### Workspace

- Rust 1.91.0, edition 2024 (see `Cargo.toml`).
- Layout: `crates/*`, assets in `assets/`, scripts in `scripts/`.

### Commands

```bash
make check      # type-check workspace
make test       # run all tests
make clippy     # lint (deny warnings)
make fmt-check  # verify formatting
make build      # release build
```

### Observability

- `mimic_snapshot()`: returns a live `StorageReport` snapshot:
  - `storage_data` and `storage_index` (store snapshots),
  - `entity_storage` (per-entity breakdown by store, using path names).
- `mimic_logs()`: returns the in-memory log buffer (oldest → newest).
- `mimic_metrics()`: returns an `EventReport` of ephemeral counters since `since_ms`:
  - `counters` (global ops/perf) and `entity_counters` (per-entity summary).
- `mimic_metrics_reset()`: clears counters and refreshes `since_ms`.

Examples
```bash
dfx canister call <canister> mimic_snapshot
dfx canister call <canister> mimic_logs
dfx canister call <canister> mimic_metrics
dfx canister call <canister> mimic_metrics_reset
```

### Versioning

We use semver with convenience scripts:

```bash
make version   # current version
make patch     # bump 0.15.2 -> 0.15.3 (updates manifests + tags)
make minor     # bump 0.15.2 -> 0.16.0 (updates manifests + tags)
make major     # bump 0.15.2 -> 1.0.0 (updates manifests + tags)
make release   # no-op; CI releases on tag push
```

---

## 🧭 Style Guide

- Naming: use "event" for runtime counters/logs (e.g., `EventReport`, `mimic_metrics`, `EventSelect`), and "snapshot" for storage views.
- Errors: prefer `?` over `unwrap()/expect()`; return typed errors with `thiserror`.
- Visibility: default to private; use `pub(crate)` unless needed publicly.
- Docs: add rustdoc to all public items; include a brief example when practical.
- Rustdoc style: use triple-slash `/// ` with a space, placed directly above important structs/enums/traits; leave a blank line before the doc block for readability.
- Functions: keep functions small and focused; extract helpers for clarity.
- Borrowing: avoid unnecessary clones; favor iterators and borrowing.
- Imports: group per crate and nest items where practical; prefer a single `use crate::{ ... }` instead of multiple `use crate::...` lines.
- Imports (std): pull common std items into scope at the top (e.g., `use std::collections::BTreeMap;`) rather than fully-qualifying inline.
- Counters: use saturating arithmetic for totals; no wrapping arithmetic.
- Perf: only optimize on proven hot paths (based on profiling); consider pre-allocation or other micro-optimizations when it clearly pays off.
- Tests: co-locate small unit tests; keep them deterministic and behavior-named.
- Compatibility: no backward-compatibility guarantees at this stage. Prefer clean breaks over shims to avoid technical debt. Document breaking changes briefly in the changelog.
- Codegen (mimic_build): generate minimal glue and delegate to `mimic::interface::*`. The generator wires automatic endpoints in each actor root and can leverage the schema to emit schema-driven pieces (paths, signatures, derives), but core logic should live in library modules.

These complement clippy + rustfmt; run `make clippy` and `make fmt-check` before opening a PR.

---

## 📊 Current Focus

* Improving docs and examples
* Better error modeling (`MimicError` + sub-errors)
* Testing index logic, filters, fixtures
* Store statistics & memory usage
* Reducing WASM size from codegen


<!-- License intentionally omitted for internal/private use. -->

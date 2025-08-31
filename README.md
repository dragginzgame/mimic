

![MSRV](https://img.shields.io/badge/rustc-1.81+-blue.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Documentation](https://docs.rs/mimic/badge.svg)](https://docs.rs/mimic)

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

It was originally built for the Web3 game [Dragginz](https://dragginz.io/) and is now open-sourced for the wider IC community. Mimic aims to make building **schemas, queries, and storage-backed entities** ergonomic, safe, and fun.

---

## ✨ Features

- **Entity macros** — define entities declaratively with schema attributes
- **Query builder** — type-safe filters, ordering, offsets, limits
- **Stable storage** — powered by `ic-stable-structures` B-Trees
- **Automatic endpoints** — `mimic_build` generates `mimic_query_load`, `mimic_query_save`, `mimic_query_delete`
- **Stats API** — optional `mimic_stats` endpoint for inspecting stores
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
        field(name = "id", value(item(is = "types::Ulid"))),
        field(name = "name", value(item(is = "text::Name"))),
        field(name = "description", value(item(is = "text::Description"))),
        field(name = "order", value(item(is = "game::Order"))),
        field(name = "color", value(item(is = "types::color::RgbHex"))),
    ),
)]
pub struct Rarity {}
````

### Query entities

```rust
#[query]
pub fn rarities(
    offset: usize,
    limit: Option<usize>,
    filter: FilterExpr,
    order: OrderExpr,
) -> Result<Vec<Rarity>, mimic::Error> {
    db().load::<Rarity>()
        .debug()
        .all()
        .offset(offset)
        .filter(filter)
        .order(order)
        .limit_option(limit)
        .execute()?
        .entities()
        .collect()
}
```

---

## 📦 Crates

* **`mimic`** — main ORM framework (entities, queries, schema, stores, types, utils).
* **`mimic_build`** — code generation for canisters (`build.rs` → `actor.rs`).
* **`mimic_common`** — shared utilities.
* **`mimic_schema`** — schema definitions and types.

---

## 🔧 Modules (in `mimic`)

* `core` — traits, keys, type system, validation.
* `db` — query execution, stores, registries, persistence.
* `design` — schema macros and design-time structures.
* `interface` — canister endpoints, errors, stats API.
* `macros` — procedural macros (`#[entity]`, `mimic_start!`, etc).
* `types` — reusable types (ULID, Cardinality, colors, etc).
* `utils` — helper libraries.

---

## 🧑‍💻 Development

### Install

See [INSTALLING.md](INSTALLING.md).

### Versioning

We use semver with convenience scripts:

```bash
make version   # current version
make patch     # bump 0.9.3 -> 0.9.4
make minor     # bump 0.9.3 -> 0.10.0
make major     # bump 0.9.3 -> 1.0.0
make release   # create release tag
```

---

## 📊 Current Focus

* Improving docs and examples
* Better error modeling (`MimicError` + sub-errors)
* Testing index logic, filters, fixtures
* Store statistics & memory usage
* Reducing WASM size from codegen


## 📜 License

MIT — see [LICENSE](LICENSE).


![MSRV](https://img.shields.io/badge/rustc-1.81+-blue.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Documentation](https://docs.rs/mimic/badge.svg)](https://docs.rs/mimic)

# Mimic Data Model Framework

![An appealing funny cover image to introduce Mimic](image.png)

```
Make It [ Matter     ] on the Internet Computer
          Magical
          Monetise
          Modular
          Multiplayer
          Mostly
          Mainstream
```

## 👋 Introduction

Hi, I'm @borovan and I LARP as a Rust developer. This is my ORM framework, originally designed for the web3 game [Dragginz](https://dragginz.io/) but we have decided to open source it to get help making it better, and also provide a tool for others to develop on the [Internet Computer](https://internetcomputer.org).

### What is Mimic?

We want to be able to design entities using a customised macro language, and then have a query builder to access the data, like this :

```rust
    /// Rarity
    /// affects the chance of an item dropping or an event occurring
    #[entity(
        sk(field = "id"),
        fields(
            field(name = "id", value(item(is = "types::Ulid"))),
            field(name = "name", value(item(is = "text::Name"))),
            field(name = "description", value(item(is = "text::Description"))),
            field(name = "order", value(item(is = "game::Order"))),
            field(name = "color", value(item(is = "types::color::RgbHex"))),
            order(field = "order", direction = "asc"),
        ),
    )]
    pub struct Rarity {}
```

```rust
// rarities
#[query]
pub fn rarities(...) -> Result<Vec<Rarity>, Error> {
    DB.with(|db| {
        let rarities = mimic::db::query::load::<Rarity>(db)
            .debug()
            .all()
            .offset(offset)
            .filter(filter)
            .order(order)
            .limit_option(limit)
            .execute()?
            .entities()
            .collect();

        Ok(rarities)
    })
}

```

### FAQ

#### How do I install Mimic?
**A:** We have an install guide [here](INSTALLING.md).

#### How do I manage versions and releases?
**A:** We have a comprehensive versioning guide [here](VERSIONING.md) and convenient tools:

```bash
# Show current version
make version

# List available git tags
make tags

# Bump versions
make patch    # 0.9.3 -> 0.9.4
make minor    # 0.9.3 -> 0.10.0  
make major    # 0.9.3 -> 1.0.0

# Create a release
make release
```

#### How do I integrate Mimic as a git dependency?
**A:** We have a comprehensive integration guide [here](INTEGRATION.md). Quick start:

```toml
[dependencies]
mimic = { git = "https://github.com/dragginzgame/mimic", tag = "v0.9.2", features = [] }
```

-------

#### Current Situation

- Documentation is a disaster because it's evolving so quickly I just make it look neat and forget about
actually writing useful documentation
- HUGE emphasis on macros which slows down the IDE but it's also what makes it so easy to write game design

#### Feature TODO

- Indexing for B-Trees (no use-case yet however)

### Testing TODO

- So far we have barely scratched the surface of how this code should be tested

-------------

## ❓Open Questions

### Crates & Modules

- feedback needed on the amount of crates in the framework. Does it have to be so many?  What's the best practice
for organising crates in a complicated project

### Errors

- what's the best way to handle a framework that has about 50 different error types?

-----

## 📦 Top-Level Crates

- `mimic` - the codebase is here, plus a top level `mimic/src` crate that includes and organises everything
- `mimic_build` - the ctor macros that allow you to build your data model or schema

### Mimic

#### config

Framework-level runtime configuration.  Magic numbers, hash seeds, directories etc.

Anything compile time we would have to pass into Mimic as an environment variable or rust feature.

#### schema

All of the schema types, schema build and state

#### ic

the Internet Computer and related repos are all wrapped in the ic crate, with additional helpers

#### query

Query interface for the database.  Contains query builders, and a schema resource locator.

#### store

The B-Tree that stores data in Mimic

#### types

General types that mimic uses, including things like Cardinality that are shared between the data
model builder and the schema

#### utils

Libraries.  Notably ulid is wrapped here and also wrapped within the ORM, just so we can use it as a raw API CandidType, and also within the ORM where it gains a whole lot more features.


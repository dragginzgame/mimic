![MSRV](https://img.shields.io/badge/rustc-1.81+-blue.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Documentation](https://docs.rs/mimic/badge.svg)](https://docs.rs/mimic)

# Mimic dApp Framework

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
        store = "canister::game_config::store::Data",
        pks = "id",
        fields(
            field(name = "id", value(item(is = "types::Ulid"))),
            field(name = "name", value(item(is = "text::Name"))),
            field(name = "description", value(item(is = "text::Description"))),
            field(name = "order", value(item(is = "game::Order"))),
            field(name = "key", value(item(is = "types::orm::EnumHash"))),
            field(name = "color", value(item(is = "types::color::RgbHex"))),
            order(field = "order", direction = "asc"),
        ),
        source = "poly::discovery::Discovery",
        crud(load(permission = "auth::permission::CrudLoad"))
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

-------

#### Current Situation

- Documentation is a disaster because it's evolving so quickly I just make it look neat and forget about
actually writing useful documentation
- HUGE emphasis on macros which slows down the IDE but it's also what makes it so easy to write game design

#### Feature TODO

- Indexing for B-Trees (no use-case yet however)
- Caching of derive entities in each canister.  So you can do all these complex queries to build a type and then cache it automatically.
- Stable structures for Cell/B-Tree, would like it if there was a few more options.  Making non-Copy Cells because of Strings seemed a bit of a stretch

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
- `mimic_base` - the base design primitives, plus a selection of validators, sanitizers and other
schema types
- `mimic_common` - common files that are used by macros at the framework level and also application level
- `mimic_derive` - derive macros (currently just Storable)

### Mimic

#### api

This crate contains helper macros for the API/Actor classes.  This is also where a lot of the errors are defined and wrapped.  As the bin/actorgen
crate generates a lot of code, this crate is mostly here to handle and organise where that code points to.

#### build

Support library for the `build.rs` scripts needed for canisters

#### config

Framework-level runtime configuration.  Magic numbers, hash seeds, directories etc.

Anything compile time we would have to pass into Mimic as an environment variable or rust feature.

#### core/schema

The runtime schema can be accessed from this crate.

#### core/state

Core-level state.  This was moved out of the actor classes as we couldn't reference it anywhere and the macro code we had to use was becoming hard to maintain.

This crate contains runtime state, which isn't great but we have a strict interface that allows access to it which somewhat lessens
the concerns.

#### core/wasm

Currently this crate contains one helper struct that allows you to store and retrieve Wasm bytes at runtime, allowing the root canister to create canisters on demand.

This logic has only been moved into a separate crate so that we can reference it via nested crates like api, and not have to rely on the actor.

#### db

The database is a collection of B-Trees.  This isn't really meant to be used directly as we wrap the database logic within a Query Builder

#### db/query

Query interface for the database.  Contains query builders, and a schema resource locator.

#### ic

the Internet Computer and related repos are all wrapped in the ic crate

#### lib

Libraries.  Notably ulid is wrapped here and also wrapped within the ORM, just so we can use it as a raw API CandidType, and also within the ORM where it gains a whole lot more features.

Do these need to be separate crates?

#### orm

[todo!()]

#### orm/macros

This is the home of all the macros that allow you to create the data model, for instance `#[entity]`, `#[newtype]`

#### orm/schema

[todo!()]

#### test

Internal test canister plus the associated schema

#### types

There are two layers of wrapping when it comes to most of the non-primitive Rust types.

Firstly we have the types like Ulid, Timestamp, Decimal.  We want to use these in endpoints (so we need serde/CandidType) but they're not ORM-ready, they don't have the
20 or so derives needed to be part of the ORM.  candid::Principal is an example of this, it derives CandidType but doesn't do much else.

The double-wrapped types are used inside the orm, in `mimic-base`.

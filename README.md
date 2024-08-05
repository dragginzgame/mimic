![MSRV](https://img.shields.io/badge/rustc-1.80+-blue.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Documentation](https://docs.rs/mimic/badge.svg)](https://docs.rs/mimic)

# Mimic dApp Framework

![An appealing funny cover image to introduce Mimic](image.png)

```
Make It [ Matter     ] on the Internet Computer
          Magical
          Monetise
          Modular
          Multiplayer
          Mainstream
          Mostly
          Moribund
          Memorable
```

## 👋 Introduction

Hi, I'm @borovan and I LARP as a Rust developer. This is my ORM framework, originally designed for the web3 game [Dragginz](https://dragginz.io/) but we have decided to open source it to get help making it better, and also provide a tool for others to develop on the [Internet Computer](https://internetcomputer.org).

## 🚧 Work In Progress

> [!NOTE]
> NOW THINGS COMPILE BUT ITS STILL A HUGE MESS 😅

### Current Situation

- Documentation is a disaster because it's evolving so quickly I just make it look neat and forget about
actually writing useful documentation
- TESTING
- HUGE emphasis on macros which slows down the IDE but it's also what makes it so easy to write game design

### Feature TODO

- Indexing for B-Trees (no use-case yet however)
- Caching of derive entities in each canister.  So you can do all these complex queries to build a type and then cache it automatically.
- Stable structures for Cell/B-Tree, would like it if there was a few more options.  Making non-Copy Cells because
of Strings seemed a bit of a stretch

### Testing TODO

- So far we have barely scratched the surface of how this code should be tested

## ❓Open Questions

> [!NOTE]
> I will move some of these to GitHub issues.

### Macros

- the `mimic_start!` macro - how should it know where the generated file is going to be?  Should I generate it into
the same directory?  Can I pass an environment variable for WORKSPACE_ROOT or something like that?

### Crates & Modules

- feedback needed on the amount of crates in the framework. Does it have to be so many?  What's the best practice
for organising crates in a complicated project

### Errors

> [!IMPORTANT]
> I warmly welcome any input on this topic!

- what's the best way to handle a framework that has about 50 different error types?

### IDE

- `rust-analyzer` gives false positives when negative numbers are used in macros, via the derive crate.  I know that
you're supposed to put them in quotes, but the ArgNumber crate works just fine.
- `rust-analyzer` false positive when including a file at the start of each actor

## 📦 Top-Level Crates

- `mimic` - the codebase is here, plus a top level `mimic/src` crate that includes and organises everything
- `mimic_base` - the design
- `mimic_common` - common files that are used by macros at the framework level and also application level
- `mimic_derive` - derive macros (currently just Storable)

### Mimic

#### api

This crate contains helper macros for the API/Actor classes.  This is also where a lot of the errors are defined and wrapped.  As the bin/actorgen
crate generates a lot of code, this crate is mostly here to handle and organise where that code points to.

#### canisters

Framework-level canisters.  Currently there's just the test canister which allows you to test things at IC runtime which cargo test can't do.

#### cli

This is the helper code to make a binary `mimicli` that can generate rust code for the actor classes, and also the
schema.json file.

`mimicli` needs to include your local design crate when compiling, so it isn't a binary in its own right.  You have to make
it into a binary yourself.

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

#### types

There are two layers of wrapping when it comes to most of the non-primitive Rust types.

Firstly we have the types like Ulid, Timestamp, Decimal.  We want to use these in endpoints (so we need serde/CandidType) but they're not ORM-ready, they don't have the
20 or so derives needed to be part of the ORM.  candid::Principal is an example of this, it derives CandidType but doesn't do much else.

The double-wrapped types are used inside the orm, in `mimic-base`.  God I suck at documentation.

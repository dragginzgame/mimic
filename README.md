![MSRV](https://img.shields.io/badge/rustc-1.80+-blue.svg) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# mimic
## <marquee>Mimic Dapp Framework</marquee>

![alt text](image.png)

```
Make It [ Matter     ] on the Internet Computer
          Magical
          Monetise
          Modular
          Multiplayer
          Mainstream
          Mostly
          Memorable
```

Hi, I'm @borovan and I LARP as a rust developer.  This is my ORM framework, originally designed for the web3 game Dragginz but we have decided to open source it to get help making it better, and also provide a tool for others to develop on the [Internet Computer](https://internetcomputer.org).


# NOTHING WORKS YET THIS IS A HUGE MESS

### Current Situation

- Documentation is a disaster because it's evolving so quickly I just make it look neat and forget about
actually writing useful documentation
- TESTING
- HUGE emphasis on macros which slows down the IDE but it's also what makes it so easy to write game design

### Notable deps

- `ctor` - this is how the schema is assembled
- `derive_more`, strum - giving our types a broad range of derived traits
- `snafu` for errors
- `serde`, `serde_json`, `serde_bytes` (no brainer).  However, had issues with serde_cbor so we have ciborium
- `ulid` for the orm (serde doesn't work with it, so wrote a custom implementation)
- `darling` for macro parsing
- `remain` for alphabetical sorting to keep OCD at bay

### Testing TODO

- So far we have barely scratched the surface of how this code should be tested

### Feature TODO

- Indexing for B-Trees (no use-case yet however)
- Caching of derive entities in each canister.  So you can do all these complex queries to build a type and then cache it automatically.
- Stable structures for Cell/B-Tree, would like it if there was a few more options.  Making non-Copy Cells because
of Strings seemed a bit of a stretch

### OPEN QUESTIONS (HELP PLZ!)

#### Crates & Modules

- strum, candid, remain - is there anyway to wrap these without requiring the dependency to be specified in
the application that's using Mimic
- comments needed on the amount of crates in the framework.  Does it have to be so many?  What's the best practice
for organising crates in a complicated project

#### Errors

- **HELP!** what's the best way to handle a framework that has about 50 different error types

#### IDE

- rust-analyzer gives false positives when negative numbers are used in macros, via the derive crate.  I know that
you're supposed to put them in quotes, but the ArgNumber crate works just fine.
- rust-analyzer false positive when including a file at the start of each actor


----------
## Top-Level Crates

- `mimic` - the codebase is here, plus a top level `mimic/src` crate that includes and organises everything
- `mimic_base` - the design
- `mimic_cli` - This contains `mimicli`, the commnand line tool to generate rust code for canister actor classes, and the schema.json file which is deserialized and used by the actors.
- `mimic_common` - common files that are used by macros at the framework level and also application level
- `mimic_derive` - derive macros (currently just Storable)

## Mimic

#### api

This crate contains helper macros for the API/Actor classes.  This is also where a lot of the errors are defined and wrapped.  As the bin/actorgen
crate generates a lot of code, this crate is mostly here to handle and organise where that code points to.

#### canisters

Framework-level canisters.  Currently there's just the test canister which allows you to test things at IC runtime which cargo test can't do.

#### db

The database is a collection of B-Trees.  This isn't really meant to be used directly as we wrap the database logic within a Query Builder

#### db/query

Query interface for the database.  Contains query builders, and a schema resource locator.

#### lib

Libraries.  Notably ulid is wrapped here and also wrapped within the ORM, just so we can use it as a raw API CandidType, and also within the ORM where it gains a whole lot more features.

Do these need to be separate crates?

#### core/config

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

#### orm

#### orm/macros

This is the home of all the macros that allow you to create the data model, for instance `#[entity]`, `#[newtype]`

#### orm/schema

#### types


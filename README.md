# mimic
Mimic Dapp Framework

( picture of fantasy game components that have been hastily been assembled, including
aforementioned mimic treasure chest monster )

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

# NOTHING WORKS YET THIS IS A HUGE MESS



### Notable deps

- ctor - this is how the schema is assembled
- derive_more, strum - giving our types a broad range of derived traits
- snafu for errors
- serde, serde_json, serde_bytes (no brainer).  However, had issues with serde_cbor so we have ciborium
- ulid for the orm (serde doesn't work with it, so wrote a custom implementation)
- darling for macro parsing
- remain for alphabetical sorting to keep OCD at bay

### (Hopefully Best) Practices

- Most larger crates wrapped for sanity / ease of imports.

### Current Situation

- documentation is a disaster because it's evolving so quickly I just make it look neat and forget about
actually writing useful documentation
- HUGE emphasis on macros which slows down the IDE but it's also what makes it so easy to write game design

### Notable TODO

- Indexing for B-Trees
- Stable structures for Cell/B-Tree, would like it if there was a few more options.  Making non-Copy Cells because
of Strings seemed a bit of a stretch
- false positives in rust-analyzer with macros.  Hopefully they go away in time.

----------

#### backend/core

This is the Dragginz Framework that doesn't yet have a name.  We'd like to eventually have this as a separate, open source app that could
be used by teams developing on the IC, much in the same way as frameworks like Symfony, Gorm, etc.  It especially suits project with complex,
evolving data models such as games.

##### backend/core/api

This crate contains helper macros for the API/Actor classes.  This is also where a lot of the errors are defined and wrapped.  As the bin/actorgen
crate generates a lot of code, this crate is mostly here to handle and organise where that code points to.

##### backend/core/bin

- schemagen - generates the schema into a schema.json file (which can be used by the frontend, and also read in by each actor class)
- actorgen - generates the rust code included by every canister actor

##### backend/core/canisters

Framework-level canisters.  Currently there's just the test canister which allows you to test things at IC runtime which cargo test can't do.

##### backend/core/config

Framework-level configuration.  Magic numbers, hash seeds, directories etc.

##### backend/core/db

The database is a collection of B-Trees.  This isn't really meant to be used directly as we wrap the database logic within a Query Builder

##### backend/core/db/query

Query interface for the database.  Contains query builders, and a schema resource locator.

##### backend/core/ic

Everything IC is wrapped in this structure.  Like a library but has it's own crate structure as it's more important (and easier to type)

##### backend/core/lib

Libraries.  Notably ulid is wrapped here and also wrapped within the ORM, just so we can use it as a raw API CandidType, and also within
the ORM where it gains a whole lot more features.

##### backend/core/orm


##### backend/core/schema


##### backend/core/service

Core-level services.  This @todo@ will probably get rolled into Schema as really all the services just load in and repackage Schema
data.  Perhaps there could be plugin services, but those could probably live in app/ land.

##### core/state

Core-level state.  This was moved out of the actor classes as we couldn't reference it anywhere and the macro code we had to use was
becoming hard to maintain.

This crate contains runtime state, which isn't great but we have a strict interface that allows access to it which somewhat lessens
the concerns.

##### core/wasm

Currently this crate contains one helper struct that allows you to store and retrieve Wasm bytes at runtime, allowing the root canister
to create canisters on demand.

This logic has only been moved into a separate crate so that we can reference it via nested crates like api, and not have to rely on the
actor.




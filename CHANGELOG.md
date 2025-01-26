# Mimic Changelog

All notable, and occasionally less notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [0.3.0]
- sanitise removed completely from the framework as it's bloated
and makes the code really complex, plus adds a lot of compile time and cycles
- redoing errors so they have specific names, ie. AuthError not auth::Error, and using context to bubble errors up
- Save, Load and Delete now come in dynamic and static/generic forms
- fixtures don't need to be a Vec<Box<dyn>> any more
- RowIterator renamed to LoadResult and cleaned up

## [0.2.4]
- inter canister DB query calls working
- removed QueryRow because it's effectively the same thing as DataRow
- changed enum_value to use ArgNumber not i32

## [0.2.3]
- breaking change to database API.  Queries are now constructed separately, the fluid method
does not start with DB
- enum_value has returned, we do have a use for it in Dragginz

## [0.2.2]
- primary keys are no more, sort keys now include the key of the local entity
- we didn't need the OnCreate hook, so removed that complex code
- sort key now has more strict validation, demanding relations are used for keys
- cleaned up the mimic build code so that errors are properly displayed in text and not debug
- integrated new version of convert_case, added Sentence case
- fixed a bug and vastly improved the error messages in validator::string::len
- indexes can now be unique

## [0.2.1]
- split load and load_dyn on query in preparation for inter-canister queries

## [0.2.0]
- all changes that make mimic compatible with the Dragginz repo v0.2
- split EntityDynamic into EntityDyn and NodeDyn
- mimic is now just two crates, as everything else has been incorporated into the main crate
- guard_update and guard_query back
- constants added to schema

## [0.1.9]
- we need to store the Path of an entity inside the db, so we can tell what data it's
supposed to be
- changed Fixture to EntityFixture, and removed source from Entity.  Added new schema type EntityExtra which allows you to specify a list of data sources
- bumped rust to 1.82 and did a clippy pass
- added an 'indirect' directive to item so you can wrap things in Box<T>

## [0.1.8]
- wrapped Db in an Rc so that it's easier to pass through to await code
- added better fixture validation, it tells you the entity path now
- introduced a schema Builder that allows you to register hooks pre-build/validate

## [0.1.7]
- refactor of entire api crate with new error handling, and grouped submodules
- moved ic to lib_ic to keep the libraries consistent
- moved the five startup functions into a single StartupManager trait with defaults
- removed mimic_common and mimic_derive as they were only used for Storable, and we need a more
flexible implementation of Storable anyway
- removed lib_cbor and added from_binary and to_binary into the ic::structures module, accessible
to whatever apps use mimic
- removed lib_time and moved it to the public ic:: crate
- removed lib_rand and moved it to ic too
- added the semver crate so we can validate Version
- Validate renamed to ValidateManual, Sanitize to SanitizeManual to clean up how those traits work
- looped api::error back into it's child errors to handle ic::api::call

## [0.1.6]
- changed the concrete error types to a (Code, String) tuple, as dealing with two tiers of
candid error variants (mimic and app) was a nightmare

## [0.1.5]
- guides were a code smell, so they've been removed.  Instead we have EnumValue
which allows you to specify a unit enum with an additional value argument that
is stored in the schema JSON
- EnumValue now returns an Option type to handle Unspecified
- Removed the Constant schema node because there are better ways to handle it
- PrimaryKey implemented for String

## [0.1.4]
- fixed a bug in api::call that was affecting calls with more than one argument

## [0.1.3]
- Horrible circular dependency in mimic_base with the Ulid type fixed
- root canister now has a schema() command that's guarded at the controller level
- tests have been moved out of mimic_base so they don't interfere with the application tests
- removed application dependency on derive-more, num-traits and strum
- the user canister path is no longer hard coded and its pulled from the schema

## [0.1.2]

- Build scripts!  No longer do we have to rely on a suite of bash scripts piping data to random
files that may not get included properly when directories change.
- mimic_start!() takes in the config file as an argument, couldn't think of a better way to do it

## [0.1.1]

- Filter now has different arguments.  It's an enum where you can specify All(String), or Fields(Vec<(String, String)>) to either search every field if it contains a string (OR), or multiple fields with multiple different strings (AND)
- Unit enums now derive Display + Hash so they can be used as map keys
- Enums can now use the variant(unspecified) directive

## [0.1.0]

- Ok finally compiling and the Dragginz application is using mimic and working.  Back to
how things were before the split.
- removed Primitive::path from schema as it's just used in generated rust code
- Constant now uses PrimitiveType and ArgNumber (for consistency, and to display the types
and values in the schema).  It also strips the numeric annotation so you don't have to match the value to the type.
- Isize and Usize are now both in PrimitiveType and ArgNumber for consistency

## [0.0.3]

- vanity release because now apps compile if they have mimic as a dependency!  go me!

## [0.0.2]

- actorgen and schemagen combined into a single `mimicli` executable
- added mimic::prelude and mimic::orm::prelude for external actor and design crates
- added a skip-validation flag to `mimicli`

## [0.0.1]

- Split off the Dragginz ORM into a separate repository (this one) and renamed it Mimic.  Everything in
this 0.0.1 release will just be getting the thing working.

# Mimic Changelog

All notable, and occasionally less notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

## [0.1.7]
- refactor of entire api crate with new error handling, and grouped submodules
- moved ic to lib_ic to keep the libraries consistent
- moved the five startup functions into a single StartupManager trait with defaults
- removed mimic_common and mimic_derive as they were only used for Storable, and we need a more
flexible implementation of Storable anyway

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
- Item now can be: id, is or rel.  Id is the addition, so you just type (value(item(id))) for a Ulid
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

# Mimic Changelog

All notable, and occasionally less notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## Unreleased

## [0.1.0]

- Ok finally compiling and the Dragginz application is using mimic and working.  Back to
how things were before the split.
- removed Primitive::path from schema as it's just used in generated rust code
- Constant now uses PrimitiveType and ArgNumber (for consistency, and to display the types
and values in the schema)

## [0.0.3]

- vanity release because now apps compile if they have mimic as a dependency!  go me!

## [0.0.2]

- actorgen and schemagen combined into a single `mimicli` executable
- added mimic::prelude and mimic::orm::prelude for external actor and design crates
- added a skip-validation flag to `mimicli`

## [0.0.1]

- Split off the Dragginz ORM into a separate repository (this one) and renamed it Mimic.  Everything in
this 0.0.1 release will just be getting the thing working.

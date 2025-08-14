# Mimic Changelog

All notable, and occasionally less notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## [0.12.0] - 2025-08-14
- Initial release

## [0.12.0] - Query Executor Generic Change
- ::<Entity> has been moved to the root of the fluent builder, which means we can make it a lot more
ergonomic.  The compiler can infer <T, U> without <E, _, _>

## [0.11.0] - New Query Language
- replaced entire query builder with a much more easy to use / ergonomic way of constructing queries.
ie. .filter(|f| (f.eq("field", 3) | f.eq("field", 4"))

## [0.10.12] - 2025-08-11
- fixed a bug with inserting into an optional index
- removed Account as a primitive type

## [0.10.4] - 2025-08-11
- made ctor anonymous and now mimic users don't have to include it (done via macro attributes)
- moved Timestamp to a primitive type / Value
- redid E8s and E18s with better API

## [0.10.0] - Indexes for All Types
- changed Index equality to use hashed Values, implemented a canonical to_bytes for all Value types
- now any type can be used as an Index
- found lots of newtype bugs with things like Nat, Int, Principal and fixed them
- ordering bug with Decimal, so made sure all types can Order

## [0.9.22] - 2025-08-09
- added a way to detect invalid indexes at compile time
- added FieldKey to subaccount so it can be used as a database key/index

## [0.9.20] - 2025-08-08
- change extract_from_index so it skips any index that produces no keys (to fix a nasty bug)

## [0.9.18] - 2025-08-08
- actually using the filter on the DeleteExecutor
- added lots of tests for deleting, also fixed the fact Filterable was being saved to the wrong Store
- updated rust to 1.89
- moved shared Load and Delete code into Context
- added db!().create::<E>(e), replace and update

## [0.9.14] - 2025-08-07
- better error message on 0 rows found
- removed len and deref from LoadCollection, now its count -> u32

## [0.9.10] - 2025-08-07
- removed second generic arg on load and delete many()

## [0.9.8] - 2025-08-06
- fixed Selector so that it tries to use a familiar ident

## [0.9.6] - 2025-08-06
- added From<&String> to Value

## [0.9.5]
- added and_eq and or_eq to the filter builder

## [0.9.4] - Versioning & Security System
- **CRITICAL SECURITY**: Implemented tag immutability - once a version is tagged, the code at that version can NEVER change
- **NEW**: Comprehensive versioning system with automated tools (`make patch`, `make minor`, `make major`)
- **NEW**: Security validation scripts (`make security-check`, `make check-versioning`)
- **NEW**: Automated CI/CD pipeline with GitHub Actions
- **NEW**: Complete documentation suite (VERSIONING.md, INTEGRATION.md, RELEASE_GUIDE.md)
- **NEW**: Git dependency integration with immutable version pinning
- **IMPROVED**: Fixed all clippy warnings with `-D warnings`
- **IMPROVED**: Code quality optimizations (collapsible if statements, unnecessary map_err removal)
- **IMPROVED**: Enhanced Makefile with comprehensive command shortcuts
- **SECURITY**: Enterprise-grade tag immutability enforcement
- **SECURITY**: Supply chain attack prevention through immutable versions
- **SECURITY**: Automated security checks and validation

## [0.9.3] - Making Saving Great Again
- save queries now return the new entity, making it easier to return results without additional queries
- attached back the automated query endpoints
- made it a bit more user friendly.  Selector Variants can have a name, not an ident, and it
generates the ident for you.

## [0.9.2]
- query rewrite, LimitExpr is separate, and all parts of the query validate
- added tests to make sure that any query with an invalid field name on filter or sort fails
- combined default and impl into TraitStrategy, because sometimes we need both
- rewrote LoadExecutor to get a proper fast-track path for count() with a filter
- LoadQuery::all()
- count() alongside execute() uses the same logic but won't deserialize unless it has to

## [0.9.1]
- added a u8 version to the serialized data in the DataStore
- now converting metadata directly into bytes to avoid candid encode/decode - 10% less wasm instructions!
- standardised filter() on both Query and Executor types
- added validators for UpperCamel and UPPER_SNAKE
- moved text::Function to ident::{Function, Variant, Field}

## [0.9.0] - Codegen Rewrite
- rewrite of the macro codegen, much faster in vscode now
- added have view(), views() and try_view() to LoadCollection
- removed EntitySearchable and FieldSearchable as Value takes care of that
- removed EntityAccessor, FieldSortable as Value also takes care of that

## [0.8.3]
- cleaned up filter api with less generics
- added filter_eq to LoadExecutor and DeleteExecutor
- changed closures in FieldAccessor to static functions, smaller wasm and faster compiles

## [0.8.2] - Maintenance & Optimisation
- reduced delete executor to just one mutable borrow
- added delete_lots test just in case
- IndexTuple::HAS_INDEXES trait constant so you can easily skip indexes if needed
- db::Db is now passed to fixtures and not the SaveExecutor
- QueryShape::All removed because it's dangerous and doesn't add anything
- Delete Range working
- removed the Inner trait as to_view makes it obsolete
- merged the ValidatorBytes, ValidatorString into just Validator
- made index key a u64 and moved the fields into the IndexEntry

## [0.8.1] - Internal Changes
- added CanisterKind, StoreKind, IndexKind so that we can declare the static types
- Entity::Indexes are now type tuples (IndexA, IndexB)
- #[index] is used to declare an index
- completely rewrote the AsMacro, AsType, AsSchema code
- EntitySearchable and EntitySortable reduced dramatically in size using a shared helper

## [0.8.0] - Feature Complete?
- the query planner is now resolving indexes and hitting the IndexStore to get the keys
it needs, this means mimic is pretty much feature complete (for now).  Now comes a lot of testing!
- added count and count_all which take advantage of the new lazy evaluation on stable structures
BTreeMap

## [0.7.3]
- shortcut methods for create, update and replace
- In, AllIn and AnyIn now should work in queries
- mul and div for decimal
- e8s and e18s a lot more stable with more tests, from Decimal etc.

## [0.7.2] - Range and Filter
- we're now using a QueryPlanner that analyses the indexes
- changed View::default() so it's much leaner and works better
- every type now has an enforced default
- redid savequery and reduced a lot of code and bloat, save now returns the key

## [0.7.1] - Split Type and Relation
- now you can specify rel= and is/prim= at the item level, so the actual
type (Ulid/Principal, not Key) is used in the struct
- view now derives Default, good for endpoint code, but really hard to implement
as Principal doesn't derive Default
- added checking for redundant indexes

## [0.7.0] - Removed Entity Nesting
- Just wasn't working out, adding more complexity.  Much simplified now
and 10% faster
- Ulid storable now 16 bytes not 28
- Added tests for all storable types to make sure we never get panics from ic stable structures

## [0.6.0] - View Layer
- All types now implement TypeView where Type::View is a simplified type that
can be used in DTOs.
- removed lots of boilerplate code for Option and Vec<T> by using traits instead
(Visitable, EntitySearch etc.)
- Complete rewrite of mimic_declare (formerly mimic_design) with much more sensible traits, and
ToTokens derived for a specific purpose
- FieldList has come back to consolidate logic over multiple Fields
- Validator now has a FieldList so the types used to validate are in the schema
- optimised the mimic_declare macros as we don't need to pass through tokens any longer

## [0.5.5]
- IndexValue types (DataKey, IndexKey) are now Copy, so no Text keys
- Every EntityKind has a PrimaryKey type [IndexValue; N].  Single PrimaryKeys can be flattened to use as an IndexValue
- FixedE8 changed to E8s, and E18s as a bonus.  PrimitiveType -> Primitive as we don't have the macro node any more.
- Adding a suite of DTO helper functions in mimic::dto, to convert design types <-> DTO types

## [0.5.4]
- moved proc_macro2, darling, syn and quote out of mimic and just into the mimic_design crate and it saved
around 150k
- merged mimic_base and mimic, and added a seperate mimic_common trait.  Now the Schema types that are shared between
mimic and mimic_design are properly wrapped and kept DRY
- redid the crate layout so that people only have to include mimic
- removed hundreds of derives on types that didn't need them
- added a FixedE8 type to use instead of u64/Decimal for token values
- added MaxDecimalPlaces validator
- cleaned up the mimic_tests, design, canister a lot

## [0.5.3]
- full support for WHERE logic trees, with simplification and DeMorgan's laws (thanks GCSE Computing!)
- added type coercion so you can search for a u64 == 4u8
- removed .search() from the query as you can do that all within where now
- removed serde_json from the main mimic package, and did a cleanup
- renamed where to filter, and we have full nested logic tree searching now

## [0.5.2]
- changed the .to_string() usage of indexes and where statements to use a variant type
- sort key renamed to data key
- removed the dynamic loader db!().load_dyn, as it's not really needed
- removed Nat128 and Int128 which shouldn't have been there, they map to Nat and Int anyway

## [0.5.1]
- redid the query_load!() macro to be a single macro mimic_query!() that returns an Executor

## [0.5.0]
- Schema is now all static!  It doesn't need to be stored in memory at runtime
- SortKey and IndexKey changed to have a u64 id for 40x faster lookups
- data directory renamed db
- all the bits in the top mimic/ that were defined, generated entity specific are in def/
- added Index creation and deleting into the IndexStore
- replaced Sha256 with xxhash3
- benchmarks properly working
- 10x speed up in Storable for DataValue as we skip serializing the vec<u8>

## [0.4.6]
- Searchable trait now a blanket implementation for Display
- removed PrimitiveGroup in favour of is_numeric, is_displayable
- flattened the types::prim directory, so its types::ErrorTree, types::Ulid etc.
- Principal now handles From and PartialEq with its wrapped type much betterer
- added Subaccount as a primitive type, so Storable sticks to 32 bytes
- using the wrapped Subaccount and ic_principal from icu
- added Account to primitives

## [0.4.5]
- grouped data, store, response and executor under data/
- removed generics from queries, and put them onto the executor instead
- made IndexStore and DataStore newtypes
- added IndexValue that is a HashMap<String> to store index values
- Relation and CompositeKey have been merged to just Key

## [0.4.4]
- massive refactor!
- Primitive types now gone, they're just an item(prim = "PrimitiveType"), and all moved to the mimic crate
- removed id() and composite_key() and instead the logic will be inside the Resolver
- FormatSortKey now derived for all types, so we can get the indexed values alongside the keys
- Node traits (code generation) are now separate from Kind traits (schema types)
- removed lots of legacy code, like traits we weren't using

## [0.4.3]
- LoadQueryDyn is now generic, dropped the path variable, and include_children is available
- moved base out of mimic and added mimic_base.  mimic_common can now go back into mimic
- Sorted moved to a trait, which altered the logic so that traits can have attributes but no impl or derive
- moved build back into mimic
- changed the Errors so only the top level MimicError derives CandidType/Serialize/Deserialize
- removed Errors from Query as when we moved the executors out it made less sense

## [0.4.2]
- made Db into StoreRegistry and made it generic.  Now you can define multiple registries (one for DataStores, one
for IndexStores)

## [0.4.1]
- filter renamed search.  Filtering queries is now done using a closure like the rest of the rust ecosystem
- added search for Cardinality::Many, performance improvements
- removed FieldList as it wasn't adding anything
- rewrote EntitySort and EntitySearch to be way more performant
- removed the two different SaveQueries to simplify it in preparation for indexes
- moved all shared types between mimic and mimic_design into mimic_common, and re-exported it from mimic

## [0.4.0]
- breaking changes!  mimic_end is gone, moved a lot of things to ICU
- no more config.toml
- merged the preludes so there's just one and its top level
- renamed macro endpoints to have mimic_ prefix
- as_ref only derived for strings and ulids now
- refactored LoadMap to have a Relation as a key, and did From<SortKey> for Relation

## [0.3.8]
- Relation isn't Vec<Ulid> any more, instead it's Vec<String> to handle principals and other types
- IntoIterator auto derived for List, Set, Map
- Removed config file as it's just extra setup pain and isn't needed

## [0.3.7]
- standardising with the ICRC standardisations.  Nat, Int, Nat8 etc.
- lots more work on the RGBA types, thanks ChatGPT
- Relations are now a whole sort key not a ulid
- enforced _id, _ids, _key, s
- renamed the primitive type String to Text to match the Candid interface
- fixed a bug where RelationSet wasn't being used (a HashSet of Relations)
- fixed a bug where Opt/Many types with embedded validators wouldn't respect the cardinality
- is = Entity would work when it shouldnt
- added MimeType

## [0.3.6]
- completely redid the type validators, and now we have field-level validators too, so you can make a
field a U8 and validate it between 10 and 100
- went through the mimic_design/imp directory and forced everything into traits to be tidy
- moved type validators into the Item struct, so they can be used on basically anything
- isize and usize make no sense so have been removed

## [0.3.5]
- Map and List are back, and the Map logic has been taken out of Newtype, it's much cleaner this way.
- Newtype's Primitive is now not optional, as it's always Cardinality::One
- Fixed a bug with Filterable not working on Ulids
- Default has moved from Value, it's now on Newtype and Field
- moved a lot of From/Into logic to derive_more
- removed the map_derive() from the design logic, need a better way to do that
- Newtypes now have a brand new From<X> that goes down all the way to the bottom primitive
- BIG rewrite of trait derives/impls in the mimic_design package.  Now they're only calculated once and the functions
return Option<TokenStream> so it's easier to fall through to a simple derive
- Maps are now proper HashMap<K, V>
- Sets are back as a HashSet<T>, they'll only work for obvious Eq-supporting types

## [0.3.4]
- the Map type has been removed.  It was only a pseudo-map anyway as it was Vec<(K, V)> in the database.
Map logic can be handled in the DTO/game logic layers instead.
- map(key = "field") has been added to newtypes, validation is automatically carried out in validate_auto
- UlidSet replaces Vec<Ulid> so its easier to copy and also removes duplicate relations

## [0.3.3]
- removed the iterator from the load query, it's not really needed and makes the code super complex
- moved the build code into a new mimic_build crate
- made load, save, delete much cleaner and following exactly the same builder pattern
- standardised LoadResponse, DeleteResponse and removed the extra Result types
- added LoadMap and id to Entity trait to prepare for DTO hash maps

## [0.3.2]
- query::load<E> now replaced with Query::<E>::load() so I can have a map of string -> Type

## [0.3.1]
- added the 'todo' option for fields and types under the subcategory ty(), also TypeNode
- rust 1.84.1
- unit type is now there if you don't specify is or rel
- rewrote Item so it has a path and an optional entity
- switched ELoadBuilder and LoadBuilder to LoadBuilder and LoadBuilderDyn, better fluent api
- composite_key now uses unwrap_or_default()
- snafu has been replaced with thiserror
- removed strum as derive_more 2.0 does it all, also now properly exporting derive_more
- added mimic_get_store and StoreError

## [0.3.0]
- removing automated IC actor scripts [HUGE CHANGE]
- sanitise removed completely from the framework as it's bloated
and makes the code really complex, plus adds a lot of compile time and cycles
- redoing errors so they have specific names, ie. AuthError not auth::Error, and using context to bubble errors up
- Save, Load and Delete now come in dynamic and static/generic forms
- fixtures don't need to be a Vec<Box<dyn>> any more
- RowIterator renamed to LoadResult and cleaned up
- moved query out of the db/ directory

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
- added mimic::prelude and mimic::prelude for external actor and design crates
- added a skip-validation flag to `mimicli`

## [0.0.1]

- Split off the Dragginz ORM into a separate repository (this one) and renamed it Mimic.  Everything in
this 0.0.1 release will just be getting the thing working.

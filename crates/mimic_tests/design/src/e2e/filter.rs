use crate::prelude::*;

///
/// Filterable
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "name", value(item(prim = "Text"))),
        field(name = "category", value(item(prim = "Text"))),
        field(name = "active", value(item(prim = "Bool"))),
        field(name = "score", value(item(prim = "Decimal"))),
        field(name = "level", value(item(prim = "Nat8"))),
        field(name = "opt_level", value(opt, item(prim = "Nat8"))),
        field(name = "offset", value(item(prim = "Int32"))),
        field(name = "tags", value(many, item(prim = "Text"))),
        field(name = "pid", value(item(prim = "Principal"))),
        field(name = "abc", value(item(is = "FilterableEnum"))),
    )
)]
pub struct Filterable {}

///
/// FilterableOpt
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "name", value(opt, item(prim = "Text"))),
        field(name = "level", value(opt, item(prim = "Nat8"))),
        field(name = "offset", value(opt, item(prim = "Int32"))),
        field(name = "pid", value(opt, item(prim = "Principal"))),
    )
)]
pub struct FilterableOpt {}

///
/// FilterableIndex
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    index(store = "TestIndexStore", fields = "name", unique),
    index(store = "TestIndexStore", fields = "name_opt"),
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "name", value(item(prim = "Text"))),
        field(name = "name_opt", value(opt, item(prim = "Text"))),
        field(name = "name_many", value(many, item(prim = "Text"))),
    )
)]
pub struct FilterableIndex {}

///
/// FilterableEnum
///

#[enum_(
    variant(ident = "A", default),
    variant(ident = "B"),
    variant(ident = "C")
)]
pub struct FilterableEnum {}

///
/// FilterableEnumFake
///

#[enum_(
    variant(ident = "A", default),
    variant(ident = "B"),
    variant(ident = "C")
)]
pub struct FilterableEnumFake {}

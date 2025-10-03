use crate::prelude::*;

///
/// Filterable
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(
        field(ident = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(ident = "name", value(item(prim = "Text"))),
        field(ident = "category", value(item(prim = "Text"))),
        field(ident = "active", value(item(prim = "Bool"))),
        field(ident = "score", value(item(prim = "Decimal"))),
        field(ident = "level", value(item(prim = "Nat8"))),
        field(ident = "opt_level", value(opt, item(prim = "Nat8"))),
        field(ident = "offset", value(item(prim = "Int32"))),
        field(ident = "tags", value(many, item(prim = "Text"))),
        field(ident = "pid", value(item(prim = "Principal"))),
        field(ident = "abc", value(item(is = "FilterableEnum"))),
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
        field(ident = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(ident = "name", value(opt, item(prim = "Text"))),
        field(ident = "level", value(opt, item(prim = "Nat8"))),
        field(ident = "offset", value(opt, item(prim = "Int32"))),
        field(ident = "pid", value(opt, item(prim = "Principal"))),
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
        field(ident = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(ident = "name", value(item(prim = "Text"))),
        field(ident = "name_opt", value(opt, item(prim = "Text"))),
        field(ident = "name_many", value(many, item(prim = "Text"))),
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

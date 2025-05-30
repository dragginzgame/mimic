use crate::prelude::*;

///
/// CreateBasic
///

#[entity(
    store = "crate::Store",
    sk(entity = "CreateBasic", field = "id"),
    field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate")
)]
pub struct CreateBasic {}

///
/// Searchable
///

#[entity(
    store = "crate::Store",
    sk(entity = "Searchable", field = "id"),
    field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
    field(name = "name", value(item(is = "types::Text"))),
    field(name = "description", value(item(is = "types::Text")))
)]
pub struct Searchable {}

///
/// Limit
///

#[entity(
    store = "crate::Store",
    sk(entity = "Limit", field = "value"),
    field(name = "value", value(item(is = "types::Nat32")))
)]
pub struct Limit {}

///
/// SortKeyOrder
///

#[entity(
    store = "crate::Store",
    sk(entity = "SortKeyOrder", field = "id"),
    field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate")
)]
pub struct SortKeyOrder {}

///
/// SortKeyA
///

#[entity(
    store = "crate::Store",
    sk(entity = "SortKeyA", field = "a_id"),
    field(name = "a_id", value(item(is = "Ulid")), default = "Ulid::generate")
)]
pub struct SortKeyA {}

///
/// SortKeyB
///

#[entity(
    store = "crate::Store",
    sk(entity = "SortKeyA", field = "a_id"),
    sk(entity = "SortKeyB", field = "b_id"),
    field(name = "a_id", value(item(is = "Ulid"))),
    field(name = "b_id", value(item(is = "Ulid")), default = "Ulid::generate")
)]
pub struct SortKeyB {}

///
/// SortKeyC
///

#[entity(
    store = "crate::Store",
    sk(entity = "SortKeyA", field = "a_id"),
    sk(entity = "SortKeyB", field = "b_id"),
    sk(entity = "SortKeyC", field = "c_id"),
    field(name = "a_id", value(item(is = "Ulid"))),
    field(name = "b_id", value(item(is = "Ulid"))),
    field(name = "c_id", value(item(is = "Ulid")), default = "Ulid::generate")
)]
pub struct SortKeyC {}

///
/// MissingFieldSmall
///

#[record(
    field(name = "a_id", value(item(is = "types::Ulid"))),
    field(name = "b_id", value(item(is = "types::Ulid"))),
    traits(add(Default))
)]
pub struct MissingFieldSmall {}

///
/// MissingFieldLarge
///

#[record(
    field(name = "a_id", value(item(is = "types::Ulid"))),
    field(name = "b_id", value(item(is = "types::Ulid"))),
    field(name = "c_id", value(item(is = "types::Ulid"))),
    traits(add(Default))
)]
pub struct MissingFieldLarge {}

///
/// ContainsBlob
///

#[entity(
    store = "crate::Store",
    sk(entity = "ContainsBlob", field = "id"),
    field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
    field(name = "bytes", value(item(is = "types::Blob")))
)]
pub struct ContainsBlob {}

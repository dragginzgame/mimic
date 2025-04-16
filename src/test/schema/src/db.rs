use mimic::orm::{base::types, prelude::*};

///
/// CreateBasic
///

#[entity(
    store = "crate::Store",
    sk(entity = "CreateBasic", field = "id"),
    fields(field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"))
)]
pub struct CreateBasic {}

///
/// Filterable
///

#[entity(
    store = "crate::Store",
    sk(entity = "Filterable", field = "id"),
    fields(
        field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
        field(name = "name", value(item(is = "types::Text"))),
        field(name = "description", value(item(is = "types::Text"))),
    )
)]
pub struct Filterable {}

///
/// Limit
///

#[entity(
    store = "crate::Store",
    sk(entity = "Limit", field = "value"),
    fields(field(name = "value", value(item(is = "types::Nat32"))))
)]
pub struct Limit {}

///
/// SortKeyOrder
///

#[entity(
    store = "crate::Store",
    sk(entity = "SortKeyOrder", field = "id"),
    fields(field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"))
)]
pub struct SortKeyOrder {}

///
/// SortKeyA
///

#[entity(
    store = "crate::Store",
    sk(entity = "SortKeyA", field = "a_id"),
    fields(field(name = "a_id", value(item(is = "Ulid")), default = "Ulid::generate"))
)]
pub struct SortKeyA {}

///
/// SortKeyB
///

#[entity(
    store = "crate::Store",
    sk(entity = "SortKeyA", field = "a_id"),
    sk(entity = "SortKeyB", field = "b_id"),
    fields(
        field(name = "a_id", value(item(is = "Ulid"))),
        field(name = "b_id", value(item(is = "Ulid")), default = "Ulid::generate"),
    )
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
    fields(
        field(name = "a_id", value(item(is = "Ulid"))),
        field(name = "b_id", value(item(is = "Ulid"))),
        field(name = "c_id", value(item(is = "Ulid")), default = "Ulid::generate"),
    )
)]
pub struct SortKeyC {}

///
/// MissingFieldSmall
///

#[record(
    fields(
        field(name = "a_id", value(item(is = "types::Ulid"))),
        field(name = "b_id", value(item(is = "types::Ulid"))),
    ),
    traits(add(Default))
)]
pub struct MissingFieldSmall {}

///
/// MissingFieldLarge
///

#[record(
    fields(
        field(name = "a_id", value(item(is = "types::Ulid"))),
        field(name = "b_id", value(item(is = "types::Ulid"))),
        field(name = "c_id", value(item(is = "types::Ulid"))),
    ),
    traits(add(Default))
)]
pub struct MissingFieldLarge {}

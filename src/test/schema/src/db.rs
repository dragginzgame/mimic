use mimic::orm::{base::types, prelude::*};

///
/// CreateBasic
///

#[entity(
    store = "crate::Store",
    sk(entity = "CreateBasic", field = "id"),
    fields(field(name = "id", value(item(is = "types::db::UlidGenerate"))))
)]
pub struct CreateBasic {}

///
/// Filterable
///

#[entity(
    store = "crate::Store",
    sk(entity = "Filterable", field = "id"),
    fields(
        field(name = "id", value(item(is = "types::db::UlidGenerate"))),
        field(name = "name", value(item(is = "types::String"))),
        field(name = "description", value(item(is = "types::String"))),
    )
)]
pub struct Filterable {}

///
/// Limit
///

#[entity(
    store = "crate::Store",
    sk(entity = "Limit", field = "value"),
    fields(field(name = "value", value(item(is = "types::U32"))))
)]
pub struct Limit {}

///
/// SortKeyOrder
///

#[entity(
    store = "crate::Store",
    sk(entity = "SortKeyOrder", field = "id"),
    fields(field(name = "id", value(item(is = "types::db::UlidGenerate"))))
)]
pub struct SortKeyOrder {}

///
/// SortKeyA
///

#[entity(
    store = "crate::Store",
    sk(entity = "SortKeyA", field = "a_id"),
    fields(field(name = "a_id", value(item(is = "types::db::UlidGenerate"))))
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
        field(name = "a_id", value(item(rel = "SortKeyA"))),
        field(name = "b_id", value(item(is = "types::db::UlidGenerate"))),
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
        field(name = "a_id", value(item(rel = "SortKeyA"))),
        field(name = "b_id", value(item(rel = "SortKeyB"))),
        field(name = "c_id", value(item(is = "types::db::UlidGenerate")))
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

use crate::{canister, prelude::*, types};

///
/// CreateBasic
///

#[entity(
    store = "canister::test::store::Data",
    pks = "id",
    fields(field(name = "id", value(item(is = "types::Ulid"))))
)]
pub struct CreateBasic {}

///
/// Filterable
///

#[entity(
    store = "base::canister::test::store::Data",
    pks = "id",
    fields(
        field(name = "id", value(item(is = "types::Ulid"))),
        field(name = "name", value(item(is = "types::String"))),
        field(name = "description", value(item(is = "types::String"))),
    )
)]
pub struct Filterable {}

///
/// Limit
///

#[entity(
    store = "canister::test::store::Data",
    pks = "value",
    fields(field(name = "value", value(item(is = "types::U32"))))
)]
pub struct Limit {}

///
/// SortKeyOrder
///

#[entity(
    store = "canister::test::store::Data",
    pks = "id",
    fields(field(name = "id", value(item(is = "types::Ulid"))))
)]
pub struct SortKeyOrder {}

///
/// SortKeyA
///

#[entity(
    store = "canister::test::store::Data",
    pks = "a_id",
    fields(field(name = "a_id", value(item(is = "types::Ulid"))))
)]
pub struct SortKeyA {}

///
/// SortKeyB
///

#[entity(
    store = "canister::test::store::Data",
    sk(entity = "SortKeyA", fields = "a_id"),
    pks = "b_id, c_id",
    fields(
        field(name = "a_id", value(item(is = "types::Ulid"))),
        field(name = "b_id", value(item(is = "types::Ulid"))),
        field(name = "c_id", value(item(is = "types::Ulid"))),
    )
)]
pub struct SortKeyB {}

///
/// SortKeyC
///

#[entity(
    store = "canister::test::store::Data",
    sk(entity = "SortKeyA", fields = "a_id"),
    sk(entity = "SortKeyB", fields = "b_id, c_id"),
    pks = "d_id, e_id, f_id",
    fields(
        field(name = "a_id", value(item(is = "types::Ulid"))),
        field(name = "b_id", value(item(is = "types::Ulid"))),
        field(name = "c_id", value(item(is = "types::Ulid"))),
        field(name = "d_id", value(item(is = "types::Ulid"))),
        field(name = "e_id", value(item(is = "types::Ulid"))),
        field(name = "f_id", value(item(is = "types::Ulid"))),
    )
)]
pub struct SortKeyC {}

///
/// MissingFieldSmall
///

#[record(
    fields(
        field(name = "a", value(item(is = "types::Ulid"))),
        field(name = "b", value(item(is = "types::Ulid"))),
    ),
    traits(add(Default))
)]
pub struct MissingFieldSmall {}

///
/// MissingFieldLarge
///

#[record(
    fields(
        field(name = "a", value(item(is = "types::Ulid"))),
        field(name = "b", value(item(is = "types::Ulid"))),
        field(name = "c", value(item(is = "types::Ulid"))),
    ),
    traits(add(Default))
)]
pub struct MissingFieldLarge {}

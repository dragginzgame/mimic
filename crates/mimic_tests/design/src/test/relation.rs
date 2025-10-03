use crate::prelude::*;

///
/// HasRelation
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(
        field(ident = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(ident = "a_id", value(item(rel = "EntityA"))),
        field(ident = "b_id", value(item(rel = "EntityB", prim = "Nat16"))),
        field(ident = "c_id", value(item(rel = "EntityC", prim = "Principal"))),
    )
)]
pub struct HasRelation {}

///
/// HasManyRelation
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(
        field(ident = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(ident = "a_ids", value(many, item(rel = "EntityA"))),
    )
)]
pub struct HasManyRelation {}

///
/// EntityA
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(field(ident = "id", value(item(prim = "Ulid")), default = "Ulid::generate"))
)]
pub struct EntityA {}

///
/// EntityB
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(field(ident = "id", value(item(prim = "Nat16"))))
)]
pub struct EntityB {}

///
/// EntityC
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(field(ident = "id", value(item(prim = "Principal"))))
)]
pub struct EntityC {}

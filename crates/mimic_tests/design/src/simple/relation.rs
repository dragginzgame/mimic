use crate::prelude::*;

///
/// HasRelation
///

#[entity(
    store = "crate::schema::FixtureStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "a_id", value(item(rel = "EntityA"))),
        field(name = "b_id", value(item(rel = "EntityB", prim = "Nat16"))),
        field(name = "c_id", value(item(rel = "EntityC", prim = "Principal"))),
    )
)]
pub struct HasRelation {}

///
/// EntityA
///

#[entity(
    store = "crate::schema::FixtureStore",
    pk = "id",
    fields(field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"))
)]
pub struct EntityA {}

///
/// EntityB
///

#[entity(
    store = "crate::schema::FixtureStore",
    pk = "id",
    fields(field(name = "id", value(item(prim = "Nat16"))))
)]
pub struct EntityB {}

///
/// EntityC
///

#[entity(
    store = "crate::schema::FixtureStore",
    pk = "id",
    fields(field(name = "id", value(item(prim = "Principal"))))
)]
pub struct EntityC {}

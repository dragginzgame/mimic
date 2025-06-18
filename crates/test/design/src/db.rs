use crate::prelude::*;

///
/// CreateBasic
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "CreateBasic", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate")
)]
pub struct CreateBasic {}

///
/// CreateBlob
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "CreateBlob", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "bytes", value(item(prim = "Blob")))
)]
pub struct CreateBlob {}

///
/// Searchable
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "Searchable", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "name", value(item(prim = "Text"))),
    field(name = "description", value(item(prim = "Text")))
)]
pub struct Searchable {}

///
/// Limit
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "Limit", field = "value"),
    field(name = "value", value(item(prim = "Nat32")))
)]
pub struct Limit {}

///
/// SortKeyOrder
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "SortKeyOrder", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate")
)]
pub struct SortKeyOrder {}

///
/// SortKeyA
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "SortKeyA", field = "a_id"),
    field(name = "a_id", value(item(prim = "Ulid")), default = "Ulid::generate")
)]
pub struct SortKeyA {}

///
/// SortKeyB
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "SortKeyA", field = "a_id"),
    sk(entity = "SortKeyB", field = "b_id"),
    field(name = "a_id", value(item(prim = "Ulid"))),
    field(name = "b_id", value(item(prim = "Ulid")), default = "Ulid::generate")
)]
pub struct SortKeyB {}

///
/// SortKeyC
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "SortKeyA", field = "a_id"),
    sk(entity = "SortKeyB", field = "b_id"),
    sk(entity = "SortKeyC", field = "c_id"),
    field(name = "a_id", value(item(prim = "Ulid"))),
    field(name = "b_id", value(item(prim = "Ulid"))),
    field(name = "c_id", value(item(prim = "Ulid")), default = "Ulid::generate")
)]
pub struct SortKeyC {}

///
/// MissingFieldSmall
///

#[record(
    field(name = "a_id", value(item(prim = "Ulid"))),
    field(name = "b_id", value(item(prim = "Ulid"))),
    traits(add(Default))
)]
pub struct MissingFieldSmall {}

///
/// MissingFieldLarge
///

#[record(
    field(name = "a_id", value(item(prim = "Ulid"))),
    field(name = "b_id", value(item(prim = "Ulid"))),
    field(name = "c_id", value(item(prim = "Ulid"))),
    traits(add(Default))
)]
pub struct MissingFieldLarge {}

///
/// ContainsBlob
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "ContainsBlob", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "bytes", value(opt, item(prim = "Blob")))
)]
pub struct ContainsBlob {}

///
/// ContainsOpts
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "ContainsOpts", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "opt_a", value(opt, item(prim = "Principal"))),
    field(name = "opt_b", value(opt, item(prim = "Principal"))),
    field(name = "opt_c", value(opt, item(prim = "Principal"))),
    field(name = "opt_d", value(opt, item(prim = "Principal"))),
    field(name = "opt_e", value(opt, item(prim = "Principal"))),
    field(name = "opt_f", value(opt, item(prim = "Principal"))),
    field(name = "opt_g", value(opt, item(prim = "Principal"))),
    field(name = "opt_h", value(opt, item(prim = "Principal"))),
    field(name = "opt_i", value(opt, item(prim = "Principal"))),
    field(name = "opt_j", value(opt, item(prim = "Principal"))),
    field(name = "opt_k", value(opt, item(prim = "Principal"))),
    field(name = "opt_l", value(opt, item(prim = "Principal")))
)]
pub struct ContainsOpts {}

///
/// ContainsManyRelations
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "ContainsManyRelations", field = "id"),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "a_keys", value(many, item(rel = "ContainsBlob"))),
    field(name = "b_keys", value(many, item(rel = "ContainsBlob"))),
    field(name = "c_keys", value(many, item(rel = "ContainsBlob"))),
    field(name = "d_keys", value(many, item(rel = "ContainsBlob"))),
    field(name = "e_keys", value(many, item(rel = "ContainsBlob"))),
    field(name = "f_keys", value(many, item(rel = "ContainsBlob"))),
    field(name = "g_keys", value(many, item(rel = "ContainsBlob"))),
    field(name = "h_keys", value(many, item(rel = "ContainsBlob"))),
    field(name = "i_keys", value(many, item(rel = "ContainsBlob"))),
    field(name = "j_keys", value(many, item(rel = "ContainsBlob")))
)]
pub struct ContainsManyRelations {}

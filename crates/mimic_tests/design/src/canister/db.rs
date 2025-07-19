use crate::prelude::*;

///
/// CreateBasic
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    fields(field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"))
)]
pub struct CreateBasic {}

///
/// CreateBlob
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "bytes", value(item(prim = "Blob")))
    )
)]
pub struct CreateBlob {}

///
/// Searchable
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "name", value(item(prim = "Text"))),
        field(name = "description", value(item(prim = "Text")))
    )
)]
pub struct Searchable {}

///
/// Limit
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "value",
    fields(field(name = "value", value(item(prim = "Nat32"))))
)]
pub struct Limit {}

///
/// DataKeyOrder
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    fields(field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"))
)]
pub struct DataKeyOrder {}

///
/// MissingFieldSmall
///

#[record(fields(
    field(name = "a_id", value(item(prim = "Ulid"))),
    field(name = "b_id", value(item(prim = "Ulid"))),
))]
pub struct MissingFieldSmall {}

///
/// MissingFieldLarge
///

#[record(fields(
    field(name = "a_id", value(item(prim = "Ulid"))),
    field(name = "b_id", value(item(prim = "Ulid"))),
    field(name = "c_id", value(item(prim = "Ulid"))),
))]
pub struct MissingFieldLarge {}

///
/// ContainsBlob
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "bytes", value(opt, item(prim = "Blob")))
    )
)]
pub struct ContainsBlob {}

///
/// ContainsOpts
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    fields(
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
    )
)]
pub struct ContainsOpts {}

///
/// ContainsManyRelations
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "a_ids", value(many, item(rel = "ContainsBlob"))),
        field(name = "b_ids", value(many, item(rel = "ContainsBlob"))),
        field(name = "c_ids", value(many, item(rel = "ContainsBlob"))),
        field(name = "d_ids", value(many, item(rel = "ContainsBlob"))),
        field(name = "e_ids", value(many, item(rel = "ContainsBlob"))),
        field(name = "f_ids", value(many, item(rel = "ContainsBlob"))),
        field(name = "g_ids", value(many, item(rel = "ContainsBlob"))),
        field(name = "h_ids", value(many, item(rel = "ContainsBlob"))),
        field(name = "i_ids", value(many, item(rel = "ContainsBlob"))),
        field(name = "j_ids", value(many, item(rel = "ContainsBlob")))
    )
)]
pub struct ContainsManyRelations {}

///
/// Index
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    index(name = "sd", store = "crate::schema::TestIndex", fields = "x"),
    index(store = "crate::schema::TestIndex", fields = "y", unique),
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "x", value(item(prim = "Int32"))),
        field(name = "y", value(item(prim = "Int32")))
    )
)]
pub struct Index {}

impl Index {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            ..Default::default()
        }
    }
}

///
/// IndexWithFixtures
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    index(name = "x", store = "crate::schema::TestIndex", fields = "x"),
    index(name = "y", store = "crate::schema::TestIndex", fields = "y", unique),
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "x", value(item(prim = "Int32"))),
        field(name = "y", value(item(prim = "Int32"))),
        field(name = "z", value(opt, item(prim = "Int32"))),
    ),
    traits(remove(EntityFixture))
)]
pub struct IndexWithFixtures {}

impl EntityFixture for IndexWithFixtures {
    fn insert_fixtures(exec: &mut SaveExecutor) {
        // First 40 entries: unique y, non-unique x, z = None
        for i in 0..40 {
            EntityService::save_fixture(
                exec,
                Self {
                    id: Ulid::generate(),
                    x: i % 10, // allow x to repeat
                    y: i,      // y is unique
                    z: None,
                },
            );
        }

        // Next 40 entries: continue unique y, z = Some(...)
        for i in 40..80 {
            EntityService::save_fixture(
                exec,
                Self {
                    id: Ulid::generate(),
                    x: i % 5,       // again allow x to repeat
                    y: i,           // still unique
                    z: Some(i * 2), // arbitrary, safe unique-ish z
                },
            );
        }

        // Final edge case: y is still unique, z doesn't conflict
        EntityService::save_fixture(
            exec,
            Self {
                id: Ulid::generate(),
                x: i32::MAX,
                y: i32::MIN,
                z: Some(0),
            },
        );
    }
}

///
/// IndexRelation
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    index(store = "crate::schema::TestIndex", fields = "rarity_id"),
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(
            name = "rarity_id",
            value(item(rel = "crate::fixture::rarity::Rarity"))
        )
    )
)]
pub struct IndexRelation {}

///
/// IndexUniqueOpt
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    index(store = "crate::schema::TestIndex", fields = "value", unique),
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "value", value(opt, item(prim = "Nat8")))
    )
)]
pub struct IndexUniqueOpt {}

use crate::prelude::*;

///
/// Index
///

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    index(store = "crate::schema::TestIndex", fields = "x"),
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
    index(store = "crate::schema::TestIndex", fields = "x",),
    index(store = "crate::schema::TestIndex", fields = "y", unique),
    index(store = "crate::schema::TestIndex", fields = "x, z"),
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

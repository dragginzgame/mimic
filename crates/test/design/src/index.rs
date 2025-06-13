use crate::prelude::*;

///
/// Index
///

#[entity(
    store = "crate::schema::TestStore",
    sk(entity = "Index", field = "a"),
    index(store = "crate::schema::TestIndex", fields = "x"),
    index(store = "crate::schema::TestIndex", fields = "y", unique),
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "x", value(item(prim = "Int32"))),
    field(name = "y", value(item(prim = "Int32"))),
    field(name = "a", value(item(is = "crate::admin::EnumA")))
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
    sk(entity = "IndexWithFixtures", field = "id"),
    index(store = "crate::schema::TestIndex", fields = "x", unique),             // unique
    index(store = "crate::schema::TestIndex", fields = "y"),                     // normal
    index(store = "crate::schema::TestIndex", fields = "x, z"),                  // compound with opt
    index(store = "crate::schema::TestIndex", fields = "y, z", unique),          // compound unique with opt
    field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
    field(name = "x", value(item(prim = "Int32"))),
    field(name = "y", value(item(prim = "Int32"))),
    field(name = "z", value(opt, item(prim = "Int32"))),
    traits(remove(EntityFixture)),
)]
pub struct IndexWithFixtures {}

impl EntityFixture for IndexWithFixtures {
    fn insert_fixtures(exec: &mut SaveExecutor) {
        for i in 0..40 {
            EntityService::save_fixture(
                exec,
                Self {
                    id: Ulid::generate(),
                    x: i,
                    y: i % 10,
                    z: None,
                },
            );
        }

        for i in 40..80 {
            EntityService::save_fixture(
                exec,
                Self {
                    id: Ulid::generate(),
                    x: i,           // unique x
                    y: i,           // repeat y (non-unique index)
                    z: Some(i + 1), // y+z is a unique
                },
            );
        }

        // edge cases
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

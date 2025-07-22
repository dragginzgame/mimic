use crate::prelude::*;

///
/// Indexable
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    index = "IndexableA",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "pid", value(item(prim = "Principal"))),
        field(name = "ulid", value(item(prim = "Ulid"))),
        field(name = "score", value(item(prim = "Nat32"))),
    ),
    traits(remove(EntityFixture))
)]
pub struct Indexable {}

impl EntityFixture for Indexable {
    fn insert_fixtures(db: Db) {
        let principals = [
            Principal::anonymous(),
            Principal::from_slice(&[1; 29]),
            Principal::from_slice(&[2; 29]),
        ];

        let ulids = [Ulid::from_u128(1), Ulid::from_u128(2), Ulid::from_u128(3)];

        let scores = [10, 20, 30, 40, 50, 60];

        // Create combinations of principal × ulid × score
        for (i, principal) in principals.iter().enumerate() {
            for (j, ulid) in ulids.iter().enumerate() {
                let score = scores[(i + j) % scores.len()];

                EntityService::save_fixture(
                    db,
                    Self {
                        pid: *principal,
                        ulid: *ulid,
                        score,
                        ..Default::default()
                    },
                );
            }
        }
    }
}

///
/// IndexableA
///

#[index(
    store = "TestIndexStore",
    entity = "Indexable",
    fields = "pid, ulid, score"
)]
pub struct IndexableA {}

///
/// NotIndexable
///

#[entity(
    store = "TestDataStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "pid", value(item(prim = "Principal"))),
        field(name = "ulid", value(item(prim = "Ulid"))),
        field(name = "score", value(item(prim = "Nat32"))),
    )
)]
pub struct NotIndexable {}

use crate::prelude::*;

#[entity(
    store = "crate::schema::TestStore",
    pk = "id",
    index(store = "crate::schema::TestIndex", fields = "group, kind, score"),
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "group", value(item(prim = "Text"))),
        field(name = "kind", value(item(prim = "Text"))),
        field(name = "status", value(item(prim = "Bool"))),
        field(name = "score", value(item(prim = "Nat32"))),
    ),
    traits(remove(EntityFixture))
)]
pub struct Indexable {}

impl EntityFixture for Indexable {
    fn insert_fixtures(exec: &mut SaveExecutor) {
        let fixtures = [
            ("alpha", "A", true, 10),
            ("alpha", "B", false, 20),
            ("beta", "A", true, 15),
            ("beta", "C", false, 25),
            ("gamma", "B", true, 30),
            ("gamma", "C", false, 5),
            ("alpha", "A", false, 50),
            ("beta", "B", true, 60),
        ];

        for (group, kind, status, score) in &fixtures {
            EntityService::save_fixture(
                exec,
                Self {
                    group: (*group).into(),
                    kind: (*kind).into(),
                    status: *status,
                    score: *score,
                    ..Default::default()
                },
            );
        }
    }
}

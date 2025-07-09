use crate::prelude::*;

///
/// Filterable
///

#[entity(
    store = "crate::schema::FixtureStore",
    primary_key = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "name", value(item(prim = "Text"))),
        field(name = "category", value(item(prim = "Text"))),
        field(name = "active", value(item(prim = "Bool"))),
        field(name = "score", value(item(prim = "Float64"))),
        field(name = "level", value(item(prim = "Nat8"))),
        field(name = "offset", value(item(prim = "Int32"))),
    ),
    traits(remove(EntityFixture))
)]
pub struct Filterable {}

impl EntityFixture for Filterable {
    fn insert_fixtures(exec: &mut SaveExecutor) {
        let fixtures = [
            ("Alpha", "A", true, 87.2, 1, -10),
            ("Beta", "B", false, 65.1, 2, 0),
            ("Gamma", "C", true, 92.5, 3, 10),
            ("Delta", "B", false, 15.3, 2, 5),
            ("Epsilon", "A", true, 75.0, 4, -5),
            ("Zeta", "C", false, 88.8, 5, 15),
            ("Eta", "B", true, 30.5, 1, 8),
            ("Theta", "A", true, 99.9, 6, -20),
            ("Iota", "C", false, 42.0, 3, 0),
            ("Kappa", "B", true, 50.0, 2, 3),
        ];

        for (name, category, active, score, level, offset) in &fixtures {
            EntityService::save_fixture(
                exec,
                Self {
                    name: (*name).into(),
                    category: (*category).into(),
                    active: *active,
                    score: *score,
                    level: *level,
                    offset: *offset,
                    ..Default::default()
                },
            );
        }
    }
}

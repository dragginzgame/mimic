use crate::prelude::*;

///
/// Filterable
///

#[entity(
    store = "crate::schema::FixtureStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "name", value(item(prim = "Text"))),
        field(name = "category", value(item(prim = "Text"))),
        field(name = "active", value(item(prim = "Bool"))),
        field(name = "score", value(item(prim = "Float64"))),
        field(name = "level", value(item(prim = "Nat8"))),
        field(name = "offset", value(item(prim = "Int32"))),
        field(name = "tags", value(many, item(prim = "Text"))),
        field(name = "pid", value(item(prim = "Principal"))),
    ),
    traits(remove(EntityFixture))
)]
pub struct Filterable {}

impl Filterable {
    #[must_use]
    pub fn dummy_principal(n: u8) -> Principal {
        Principal::from_slice(&[n; 29])
    }
}

impl EntityFixture for Filterable {
    fn insert_fixtures(db: Db) {
        let fixtures = [
            ("Alpha", "A", true, 87.2, 1, -10, vec!["red", "blue"], 1),
            ("Beta", "B", false, 65.1, 2, 0, vec!["green"], 2),
            ("Gamma", "C", true, 92.5, 3, 10, vec!["red", "yellow"], 3),
            ("Delta", "B", false, 15.3, 2, 5, vec![], 4),
            ("Epsilon", "A", true, 75.0, 4, -5, vec!["green", "blue"], 5),
            ("Zeta", "C", false, 88.8, 5, 15, vec!["purple"], 6),
            ("Eta", "B", true, 30.5, 1, 8, vec!["red"], 7),
            ("Theta", "A", true, 99.9, 6, -20, vec!["blue", "green"], 8),
            ("Iota", "C", false, 42.0, 3, 0, vec!["yellow", "red"], 9),
            ("Kappa", "B", true, 50.0, 2, 3, vec!["green", "blue"], 10),
        ];
        for (name, category, active, score, level, offset, tags, pid_index) in &fixtures {
            EntityService::save_fixture(
                db,
                Self {
                    name: (*name).into(),
                    category: (*category).into(),
                    active: *active,
                    score: *score,
                    level: *level,
                    offset: *offset,
                    tags: tags.iter().map(ToString::to_string).collect(),
                    pid: Filterable::dummy_principal(*pid_index),
                    ..Default::default()
                },
            );
        }
    }
}

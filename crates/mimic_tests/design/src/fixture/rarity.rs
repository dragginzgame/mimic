use crate::prelude::*;

///
/// Rarity
/// example from Dragginz
///

#[entity(
    store = "crate::schema::FixtureStore",
    data_key(entity = "Rarity", field = "id"),
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "name", value(item(prim = "Text"))),
        field(name = "description", value(item(prim = "Text"))),
        field(name = "level", value(item(prim = "Nat8"))),
    ),
    traits(remove(EntityFixture))
)]
pub struct Rarity {}

impl EntityFixture for Rarity {
    fn insert_fixtures(exec: &mut SaveExecutor) {
        use RarityId as Id;

        let data = [
            (Id::Common, "Common", 1),
            (Id::Uncommon, "Uncommon", 2),
            (Id::Rare, "Rare", 3),
            (Id::Epic, "Epic", 4),
            (Id::Legendary, "Legendary", 5),
            (Id::Mythical, "Mythical", 6),
            (Id::Inconceivable, "Inconceivable", 7),
        ];

        for (id, name, level) in data {
            EntityService::save_fixture(
                exec,
                Self {
                    id: id.into(),
                    name: name.into(),
                    level,
                    ..Default::default()
                },
            );
        }
    }
}

///
/// RarityId
///

#[entity_id(
    key = "Common",
    key = "Uncommon",
    key = "Rare",
    key = "Epic",
    key = "Legendary",
    key = "Mythical",
    key = "Inconceivable"
)]
pub struct RarityId {}

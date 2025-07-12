use crate::prelude::*;

///
/// Rarity
/// example from Dragginz
///

#[entity(
    store = "crate::schema::FixtureStore",
    pk = "id",
    fields(
        field(name = "id", value(item(prim = "Ulid")), default = "Ulid::generate"),
        field(name = "name", value(item(prim = "Text"))),
        field(name = "description", value(item(prim = "Text"))),
        field(name = "level", value(item(prim = "Nat8"))),
        field(name = "color", value(item(is = "types::color::RgbHex"))),
    ),
    traits(remove(EntityFixture))
)]
pub struct Rarity {}

impl EntityFixture for Rarity {
    fn insert_fixtures(exec: &mut SaveExecutor) {
        use RarityId as Id;

        let data = [
            (Id::Common, "Common", 1, "111111"),
            (Id::Uncommon, "Uncommon", 2, "222222"),
            (Id::Rare, "Rare", 3, "333333"),
            (Id::Epic, "Epic", 4, "444444"),
            (Id::Legendary, "Legendary", 5, "555555"),
            (Id::Mythical, "Mythical", 6, "666666"),
            (Id::Inconceivable, "Inconceivable", 7, "777777"),
        ];

        for (id, name, level, color) in data {
            EntityService::save_fixture(
                exec,
                Self {
                    id: id.into(),
                    name: name.into(),
                    level,
                    color: color.into(),
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

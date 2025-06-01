use crate::prelude::*;

///
/// Rarity
/// example from Dragginz
///

#[entity(
    store = "crate::Store",
    sk(entity = "Rarity", field = "id"),
    field(name = "id", value(item(is = "Ulid")), default = "Ulid::generate"),
    field(name = "name", value(item(is = "Text"))),
    field(name = "description", value(item(is = "Text"))),
    field(name = "level", value(item(is = "Nat8"))),
    traits(remove(EntityFixture))
)]
pub struct Rarity {}

impl EntityFixture for Rarity {
    fn fixtures() -> FixtureList {
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

        let mut fixtures = FixtureBuilder::new();
        for (id, name, level) in data {
            fixtures.push(Self {
                id: id.into(),
                name: name.into(),
                level,
                ..Default::default()
            });
        }

        fixtures.build()
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

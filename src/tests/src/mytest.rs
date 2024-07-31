use base::types::Ulid;
use design::game;
use orm::traits::{EntityDynamic, Path, Validate};

//
// just a place to mess around with tests while developing
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_fixture() {
        use design::game::prop::{
            attribute::{station::SkillEntry, Attribute},
            Prop,
        };

        // create prop
        let skills = vec![SkillEntry::new(
            Ulid::from_string("00W5WCJQJ9YYEZ684GX4W3QXWW").unwrap(),
            0.into(),
        )];
        let e = Prop {
            attribute: Some(Attribute::new_station(skills)),
            ..Default::default()
        };
        println!("{e:?}");

        let se = orm::serialize(&e).expect("serializes");
        println!("serialize : {se:?}");

        let de: Prop = orm::deserialize(&se).expect("deserializes");
        println!("deserialize : {de:?}");

        let errs = e.validate();
        println!("{errs:?}");
    }

    #[test]
    fn test_entity() {
        use game::rarity::Rarity;

        // Rarity
        let mut e = Rarity {
            name: "   a  categorization of  jeweled endives!".into(),
            description: " desc ".into(),
            color: "ffffffs".into(),
            ..Default::default()
        };

        // ORM -> SANITIZE
        e.on_create();
        orm::sanitize(&mut e);
        let errs = orm::validate(&e);

        println!("errors: {errs:?}");
        println!("path : {}", Rarity::PATH);

        let se = orm::serialize(&e).expect("serializes");
        println!("serialize : {se:?}");

        let de: Rarity = orm::deserialize(&se).expect("deserializes");
        println!("deserialize : {de:?}");
    }

    #[test]
    fn test_chunk_config() {
        use design::world::chunk::Config as ChunkConfig;
        // Chunk
        let mut e = ChunkConfig {
            x: (-160).into(),
            y: 240.into(),
            z: (-8).into(),
            ..Default::default()
        };

        // ORM -> SANITIZE
        orm::sanitize(&mut e);
        let errs = orm::validate(&e);
        println!("errors: {errs:?}");

        let se = orm::serialize::<ChunkConfig>(&e).expect("serializes");
        println!("serialize : {se:?}");

        let de: ChunkConfig = orm::deserialize(&se).expect("deserializes");
        println!("deserialize : {de:?}");
    }
}

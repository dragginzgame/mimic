use crate::DB;
use mimic::{
    orm::traits::Path,
    orm::{deserialize, serialize},
    prelude::*,
    query::{self, types::Order},
};
use test_schema::Store;

///
/// DbTester
///

pub struct DbTester {}

impl DbTester {
    // test
    // best if these are kept in code order so we can see where it failed
    pub fn test() {
        Self::clear();
        Self::create();
        Self::create_lots();
        Self::data_key_order();
        Self::entity_with_map();
        Self::filter_query();
        Self::limit_query();
        Self::missing_field();
    }

    //
    // TESTS
    //

    // clear
    fn clear() {
        use test_schema::db::CreateBasic;

        // Insert rows
        for _ in 0..100 {
            let e = CreateBasic::default();
            query::create::<CreateBasic>()
                .from_entity(e)
                .unwrap()
                .execute::<CreateBasic>(&DB)
                .unwrap();
        }

        // clear
        DB.with(|db| {
            db.with_store_mut(Store::PATH, |store| store.clear()).ok();
        });

        // Retrieve the count of keys (or entities) from the store
        let count = query::load_dyn::<CreateBasic>()
            .all()
            .execute(&DB)
            .unwrap()
            .count();

        assert_eq!(count, 0, "Expected 0 keys in the store");
    }

    // create
    fn create() {
        use test_schema::db::CreateBasic;

        // clear
        DB.with(|db| {
            db.with_store_mut(Store::PATH, |store| store.clear()).ok();
        });

        let e = CreateBasic::default();
        query::create_dyn().from_entity(e).execute(&DB).unwrap();

        // count keys
        assert_eq!(
            query::load_dyn::<CreateBasic>()
                .all()
                .debug()
                .execute(&DB)
                .unwrap()
                .count(),
            1
        );

        // insert another
        let e = CreateBasic::default();
        query::create_dyn().from_entity(e).execute(&DB).unwrap();

        // count keys
        assert_eq!(
            query::load_dyn::<CreateBasic>()
                .all()
                .execute(&DB)
                .unwrap()
                .count(),
            2
        );
    }

    // create_lots
    fn create_lots() {
        use test_schema::db::CreateBasic;
        const ROWS: usize = 1_000;

        // clear
        DB.with(|db| {
            db.with_store_mut(Store::PATH, |store| store.clear()).ok();
        });

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBasic::default();
            query::create_dyn().from_entity(e).execute(&DB).unwrap();
        }

        // Retrieve the count from the store
        let count = query::load_dyn::<CreateBasic>()
            .all()
            .execute(&DB)
            .unwrap()
            .count();

        // Assert that the count matches the expected number
        assert_eq!(count, ROWS, "Expected {ROWS} keys in the store");
    }

    // data_key_order
    fn data_key_order() {
        use test_schema::db::SortKeyOrder;

        const ROWS: u16 = 1_000;

        // clear
        DB.with(|db| {
            db.with_store_mut(Store::PATH, |store| store.clear()).ok();
        });

        // Insert rows
        for _ in 1..ROWS {
            let e = SortKeyOrder::default();
            query::create_dyn().from_entity(e).execute(&DB).unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = query::load::<SortKeyOrder>()
            .all()
            .order(Order::from(vec!["id"]))
            .execute::<SortKeyOrder>(&DB)
            .unwrap()
            .keys();

        // Verify the order
        for i in 0..(keys.len() - 1) {
            assert!(
                keys[i] < keys[i + 1],
                "key ordering is incorrect at index {i}"
            );
        }
    }

    // entity_with_map
    fn entity_with_map() {
        use test_schema::map::HasMap;

        // create with map data
        let mut e = HasMap::default();
        e.map_int_string.push((3, "value".to_string()));
        e.map_int_string.push((4, "value".to_string()));
        query::create_dyn().from_entity(e).execute(&DB).unwrap();

        // load all keys
        let count = query::load_dyn::<HasMap>()
            .only()
            .execute(&DB)
            .unwrap()
            .count();

        assert!(count == 1);
    }

    // filter_query
    fn filter_query() {
        use test_schema::db::Filterable;

        // clear
        DB.with(|db| {
            db.with_store_mut(Store::PATH, |store| store.clear()).ok();
        });

        // Test data
        let test_entities = vec![
            (
                "01HMBEJJM0D6CMABQ3ZF6TMQ1M",
                "The Book of Magic",
                "This book has so much magic in it",
            ),
            (
                "01HMBEK2QT84T2GNV2Y02ED9D9",
                "The Sparkle Sword",
                "*** SPARKLES ***",
            ),
            ("01HMBEK9HQTTH6ZWYYYNTJ4ZC1", "Fruit Salad", "Yummy yummy"),
            ("01HMBEKHXZMRYDHP51APS9791H", "Same", "Same"),
        ];

        // replace
        // so that the IDs are left unchanged
        for (id, name, description) in test_entities {
            let e = Filterable {
                id: Ulid::from_str(id).unwrap(),
                name: name.into(),
                description: description.into(),
            };

            query::replace_dyn().from_entity(e).execute(&DB).unwrap();
        }

        // Array of tests with expected number of matching rows
        let tests = vec![
            ("a", 4),
            ("the", 2),
            ("Yummy", 1),
            ("yummy", 1),
            ("SPARKLE", 1),
            ("ZZXX", 0),
            ("hMbE", 4),
            ("01hmbek9", 1),
            ("same", 1),
            ("00", 0),
        ];

        for (search, expected_count) in tests {
            let count = query::load::<Filterable>()
                .all()
                .filter_all(search)
                .execute::<Filterable>(&DB)
                .unwrap()
                .count();

            assert_eq!(
                count, expected_count,
                "Test for string '{search}' in [Filter] failed",
            );
        }
    }

    // limit_query
    fn limit_query() {
        use test_schema::db::Limit;

        // clear
        DB.with(|db| {
            db.with_store_mut(Store::PATH, |store| store.clear()).ok();
        });

        // Insert 100 rows
        // overwrite the ulid with replace()
        for value in 1..100 {
            let e = Limit { value };
            query::replace_dyn()
                .from_entity(e)
                .debug()
                .execute(&DB)
                .unwrap();
        }

        // Test various limits and offsets
        for limit in [10, 20, 50] {
            for offset in [0, 5, 10] {
                let count = query::load_dyn::<Limit>()
                    .all()
                    .offset(offset)
                    .limit(limit)
                    .execute(&DB)
                    .unwrap()
                    .count();

                assert_eq!(count, limit as usize, "{limit} not equal to {count}");
                //    if !results.is_empty() {
                //        assert_eq!(results[0].value, offset + 1);
                //    }
            }
        }
    }

    // missing_field
    fn missing_field() {
        use test_schema::db::{MissingFieldLarge, MissingFieldSmall};

        let small = MissingFieldSmall {
            a_id: Ulid::generate(),
            b_id: Ulid::generate(),
        };

        // move from small to large
        let bytes = serialize(&small).unwrap();
        let large = deserialize::<MissingFieldLarge>(&bytes).unwrap();

        assert!(!large.a_id.is_nil());
        assert!(!large.b_id.is_nil());
        assert!(large.c_id.is_nil());
    }
}

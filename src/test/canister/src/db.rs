use super::STORE;
use mimic::{
    orm::{deserialize, serialize},
    prelude::*,
    query::{types::Order, Query},
};

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
            Query::create().from_entity(e).execute(&STORE).unwrap();
        }

        // clear
        STORE.with_borrow_mut(|store| {
            store.clear();
        });

        // Retrieve the count of keys (or entities) from the store
        let count = Query::<CreateBasic>::load()
            .all()
            .execute(&STORE)
            .unwrap()
            .count();

        assert_eq!(count, 0, "Expected 0 keys in the store");
    }

    // create
    fn create() {
        use test_schema::db::CreateBasic;

        // clear
        STORE.with_borrow_mut(|store| store.clear());

        let e = CreateBasic::default();
        Query::create().from_entity(e).execute(&STORE).unwrap();

        // count keys
        assert_eq!(
            Query::<CreateBasic>::load()
                .debug()
                .all()
                .execute(&STORE)
                .unwrap()
                .count(),
            1
        );

        // insert another
        let e = CreateBasic::default();
        Query::create().from_entity(e).execute(&STORE).unwrap();

        // count keys
        assert_eq!(
            Query::<CreateBasic>::load()
                .all()
                .execute(&STORE)
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
        STORE.with_borrow_mut(|store| store.clear());

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBasic::default();
            Query::create().from_entity(e).execute(&STORE).unwrap();
        }

        // Retrieve the count from the store
        let count = Query::<CreateBasic>::load()
            .all()
            .execute(&STORE)
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
        STORE.with_borrow_mut(|store| store.clear());

        // Insert rows
        for _ in 1..ROWS {
            let e = SortKeyOrder::default();
            Query::create().from_entity(e).execute(&STORE).unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = Query::<SortKeyOrder>::load()
            .all()
            .order(Order::from(vec!["id"]))
            .execute(&STORE)
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
        Query::<HasMap>::create()
            .from_entity(e)
            .execute(&STORE)
            .unwrap();

        // load all keys
        let res = Query::<HasMap>::load().only().execute(&STORE).unwrap();

        assert!(res.count() == 1);
    }

    // filter_query
    fn filter_query() {
        use test_schema::db::Filterable;

        // clear
        STORE.with_borrow_mut(|store| store.clear());

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

            Query::replace().from_entity(e).execute(&STORE).unwrap();
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
            let count = Query::<Filterable>::load()
                .all()
                .filter_all(search)
                .execute(&STORE)
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
        STORE.with_borrow_mut(|store| store.clear());

        // Insert 100 rows
        // overwrite the ulid with replace()
        for value in 1..100 {
            let e = Limit { value };
            Query::replace()
                .debug()
                .from_entity(e)
                .execute(&STORE)
                .unwrap();
        }

        // Test various limits and offsets
        for limit in [10, 20, 50] {
            for offset in [0, 5, 10] {
                let res = Query::<Limit>::load()
                    .all()
                    .offset(offset)
                    .limit(limit)
                    .execute(&STORE)
                    .unwrap();

                let count = res.count();
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

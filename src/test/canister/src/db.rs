use crate::DB;
use mimic::{
    deserialize,
    prelude_actor::*,
    query::{self, types::Order},
    serialize,
    traits::Path,
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
        let tests: Vec<(&str, fn())> = vec![
            ("blob", Self::blob),
            ("create", Self::create),
            ("create_lots", Self::create_lots),
            ("data_key_order", Self::data_key_order),
            ("search_query", Self::search_query),
            ("limit_query", Self::limit_query),
            ("missing_field", Self::missing_field),
        ];

        for (name, test_fn) in tests {
            println!("clearing db");
            DB.with(|db| {
                db.with_store_mut(Store::PATH, |store| store.clear()).ok();
            });

            println!("Running test: {name}");
            test_fn();
        }
    }

    //
    // TESTS
    //

    // blob
    fn blob() {
        use test_schema::db::ContainsBlob;

        const ROWS: u16 = 100;

        // Insert rows
        for _ in 1..ROWS {
            let e = ContainsBlob::default();
            query::create_dyn().from_entity(e).execute(&DB).unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = query::load::<ContainsBlob>()
            .all()
            .order(Order::from(vec!["id"]))
            .execute(&DB)
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

    // create
    fn create() {
        use test_schema::db::CreateBasic;

        let e = CreateBasic::default();
        query::create_dyn().from_entity(e).execute(&DB).unwrap();

        // count keys
        assert_eq!(
            query::load_dyn()
                .all(CreateBasic::PATH)
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
            query::load_dyn()
                .all(CreateBasic::PATH)
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

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBasic::default();
            query::create_dyn().from_entity(e).execute(&DB).unwrap();
        }

        // Retrieve the count from the store
        let count = query::load_dyn()
            .all(CreateBasic::PATH)
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

        // Insert rows
        for _ in 1..ROWS {
            let e = SortKeyOrder::default();
            query::create_dyn().from_entity(e).execute(&DB).unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = query::load::<SortKeyOrder>()
            .all()
            .order(Order::from(vec!["id"]))
            .execute(&DB)
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

    // search_query
    fn search_query() {
        use test_schema::db::Searchable;

        // Seed test data
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

        for (id, name, description) in test_entities {
            let e = Searchable {
                id: Ulid::from_str(id).unwrap(),
                name: name.into(),
                description: description.into(),
            };

            query::replace_dyn()
                .from_entity(e)
                .debug()
                .execute(&DB)
                .unwrap();
        }

        // Each test is: field filters -> expected match count
        let tests = vec![
            // Single field match
            (vec![("name", "Sparkle")], 1),
            (vec![("description", "SPARKLES")], 1),
            // Partial string, lowercased
            (vec![("name", "fruit")], 1),
            (vec![("description", "yummy")], 1),
            // Case-insensitive full match
            (vec![("name", "SAME")], 1),
            (vec![("description", "same")], 1),
            // Must match both fields
            (vec![("name", "same"), ("description", "same")], 1), // ✅ match
            (vec![("name", "same"), ("description", "wrong")], 0), // ❌ fails AND
            // ULID prefix
            (vec![("id", "01HMBEK9")], 1),
            // All fields must match
            (
                vec![
                    ("id", "01HMBEK9"),
                    ("name", "fruit"),
                    ("description", "yummy"),
                ],
                1,
            ),
            (
                vec![
                    ("id", "01HMBEK9"),
                    ("name", "fruit"),
                    ("description", "WRONG"),
                ],
                0,
            ),
            // No matches
            (vec![("name", "unknown")], 0),
            (vec![("name", "the"), ("description", "sparkle")], 1), // both exist, but not in same entity
        ];

        for (fields, expected) in tests {
            let search = fields
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<Vec<_>>();

            let count = query::load::<Searchable>()
                .all()
                .search(&search)
                .execute(&DB)
                .unwrap()
                .count();

            assert_eq!(
                count, expected,
                "search_fields test failed with criteria: {:?}",
                search
            );
        }
    }

    // limit_query
    fn limit_query() {
        use test_schema::db::Limit;

        // Insert 100 rows
        // overwrite the ulid with replace()
        for value in 1..100 {
            let e = Limit { value };
            query::replace_dyn().from_entity(e).execute(&DB).unwrap();
        }

        // Test various limits and offsets
        for limit in [10, 20, 50] {
            for offset in [0, 5, 10] {
                let count = query::load_dyn()
                    .all(Limit::PATH)
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

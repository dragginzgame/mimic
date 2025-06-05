use crate::{DATA_REGISTRY, INDEX_REGISTRY};
use mimic::{
    deserialize, prelude::*, query, schema::types::SortDirection, serialize, traits::Path,
    types::prim::Ulid,
};
use test_design::schema::TestStore;

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
            ("limit_query", Self::limit_query),
            ("missing_field", Self::missing_field),
            ("search_query", Self::search_query),
        ];

        for (name, test_fn) in tests {
            println!("clearing db");
            DATA_REGISTRY.with(|reg| {
                reg.with_store_mut(TestStore::PATH, |store| store.clear())
                    .ok();
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
        use test_design::db::ContainsBlob;

        const ROWS: u16 = 100;

        // Insert rows
        for _ in 1..ROWS {
            let e = ContainsBlob::default();
            query_save!().execute(query::create().entity(e)).unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = query_load!()
            .execute(
                query::load::<ContainsBlob>()
                    .all()
                    .sort_field("id", SortDirection::Asc),
            )
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
        use test_design::db::CreateBasic;

        let e = CreateBasic::default();
        query_save!().execute(query::create().entity(e)).unwrap();

        // count keys
        let num_keys = query_load!()
            .execute(query::load::<CreateBasic>().all())
            .unwrap()
            .count();
        assert_eq!(num_keys, 1);

        // insert another
        let e = CreateBasic::default();
        query_save!().execute(query::create().entity(e)).unwrap();

        // count keys

        assert_eq!(
            query_load!()
                .execute(query::load::<CreateBasic>().all())
                .unwrap()
                .count(),
            2
        );
    }

    // create_lots
    fn create_lots() {
        use test_design::db::CreateBasic;
        const ROWS: usize = 1_000;

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBasic::default();
            query_save!().execute(query::create().entity(e)).unwrap();
        }

        // Retrieve the count from the store
        let count = query_load!()
            .execute(query::load::<CreateBasic>().all())
            .unwrap()
            .count();

        // Assert that the count matches the expected number
        assert_eq!(count, ROWS, "Expected {ROWS} keys in the store");
    }

    // data_key_order
    fn data_key_order() {
        use test_design::db::SortKeyOrder;

        const ROWS: u16 = 1_000;

        // Insert rows
        for _ in 1..ROWS {
            let e = SortKeyOrder::default();
            query_save!().execute(query::create().entity(e)).unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = query_load!()
            .execute(
                query::load::<SortKeyOrder>()
                    .all()
                    .sort([("id", SortDirection::Asc)]),
            )
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

    // limit_query
    fn limit_query() {
        use test_design::db::Limit;

        // Insert 100 rows
        // overwrite the ulid with replace()
        for value in 1..100 {
            let e = Limit { value };
            query_save!().execute(query::replace().entity(e)).unwrap();
        }

        // Test various limits and offsets
        for limit in [10, 20, 50] {
            for offset in [0, 5, 10] {
                let count = query_load!()
                    .execute(query::load::<Limit>().all().offset(offset).limit(limit))
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
        use test_design::db::{MissingFieldLarge, MissingFieldSmall};

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

    // search_query
    fn search_query() {
        use test_design::db::Searchable;

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

            query_save!().execute(query::replace().entity(e)).unwrap();
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

            let count = query_load!()
                .execute(query::load::<Searchable>().all().search(search.clone()))
                .unwrap()
                .count();

            assert_eq!(
                count, expected,
                "search_fields test failed with criteria: {search:?}",
            );
        }
    }
}

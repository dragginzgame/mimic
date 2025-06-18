use crate::{DATA_REGISTRY, INDEX_REGISTRY};
use icu::perf;
use mimic::{
    db::query,
    def::{deserialize, serialize, traits::Path},
    prelude::*,
    types::Ulid,
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
            ("create_and_delete_index", Self::create_and_delete_index),
            ("create_lots", Self::create_lots),
            ("create_lots_blob", Self::create_lots_blob),
            ("data_key_order", Self::data_key_order),
            ("limit_query", Self::limit_query),
            ("missing_field", Self::missing_field),
            ("perf_options", Self::perf_options),
            ("perf_many_relations", Self::perf_many_relations),
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
            mimic_query!()
                .save()
                .execute(query::create().entity(e))
                .unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = mimic_query!()
            .load()
            .debug()
            .execute::<ContainsBlob>(query::load().all().sort_field("id", SortDirection::Asc))
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
        mimic_query!()
            .save()
            .execute(query::create().entity(e))
            .unwrap();

        // count keys
        let num_keys = mimic_query!()
            .load()
            .execute::<CreateBasic>(query::load().all())
            .unwrap()
            .count();
        assert_eq!(num_keys, 1);

        // insert another
        let e = CreateBasic::default();
        mimic_query!()
            .save()
            .execute(query::create().entity(e))
            .unwrap();

        // count keys

        assert_eq!(
            mimic_query!()
                .load()
                .execute::<CreateBasic>(query::load().all())
                .unwrap()
                .count(),
            2
        );
    }

    // create_and_delete_index
    fn create_and_delete_index() {
        use test_design::index::Index;

        // Step 1: Insert entity e1 with x=1, y=10
        let e1 = Index::new(1, 10);
        mimic_query!()
            .save()
            .debug()
            .execute(query::create().entity(e1.clone()))
            .unwrap();

        // Step 2: Insert entity e2 with x=1 (non-unique), y=20 (unique)
        let e2 = Index::new(1, 20);
        mimic_query!()
            .save()
            .debug()
            .execute(query::create().entity(e2))
            .unwrap();

        // Step 3: Attempt to insert another with duplicate y=10 (should fail)
        let e3 = Index::new(2, 10);
        let result = mimic_query!()
            .save()
            .debug()
            .execute(query::create().entity(e3.clone()));
        assert!(result.is_err(), "Expected unique index violation on y=10");

        // Step 4: Delete e1 (y=10)
        mimic_query!()
            .delete()
            .debug()
            .execute::<Index>(query::delete().one(vec![e1.id]))
            .unwrap();

        // Step 5: Try inserting e3 again (y=10 should now be free)
        let result = mimic_query!()
            .save()
            .debug()
            .execute(query::create().entity(e3));
        assert!(
            result.is_ok(),
            "Expected insert to succeed after y=10 was freed by delete"
        );

        // Step 6: Confirm only 2 entities remain
        let all = mimic_query!()
            .load()
            .debug()
            .execute::<Index>(query::load().all())
            .unwrap();

        assert_eq!(all.count(), 2);
    }

    // create_lots
    fn create_lots() {
        use test_design::db::CreateBasic;
        const ROWS: u32 = 1_000;

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBasic::default();
            mimic_query!()
                .save()
                .execute(query::create().entity(e))
                .unwrap();
        }

        // Retrieve the count from the store
        let count = mimic_query!()
            .load()
            .execute::<CreateBasic>(query::load().all())
            .unwrap()
            .count();

        // Assert that the count matches the expected number
        assert_eq!(count, ROWS, "Expected {ROWS} keys in the store");
    }

    // create_lots_blob
    fn create_lots_blob() {
        use test_design::db::CreateBlob;
        const ROWS: u32 = 500;
        const BLOB_SIZE: usize = 1024 * 2;

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBlob {
                bytes: vec![0u8; BLOB_SIZE].into(),
                ..Default::default()
            };
            mimic_query!()
                .save()
                .execute(query::create().entity(e))
                .unwrap();
        }

        // Retrieve the count from the store
        let count = mimic_query!()
            .load()
            .execute::<CreateBlob>(query::load().all())
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
            mimic_query!()
                .save()
                .execute(query::create().entity(e))
                .unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = mimic_query!()
            .load()
            .execute::<SortKeyOrder>(query::load().all().sort([("id", SortDirection::Asc)]))
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
            mimic_query!()
                .save()
                .execute(query::replace().entity(e))
                .unwrap();
        }

        // Test various limits and offsets
        for limit in [10, 20, 50] {
            for offset in [0, 5, 10] {
                let count = mimic_query!()
                    .load()
                    .execute::<Limit>(query::load().all().offset(offset).limit(limit))
                    .unwrap()
                    .count();

                assert_eq!(count, limit, "{limit} not equal to {count}");
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

    // perf_options
    fn perf_options() {
        use test_design::db::ContainsOpts;

        perf!("start perf_options");

        const ROWS: u16 = 500;

        // Insert rows
        for _ in 1..ROWS {
            let e = ContainsOpts::default();
            mimic_query!()
                .save()
                .execute(query::create().entity(e))
                .unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = mimic_query!()
            .load()
            .debug()
            .execute::<ContainsOpts>(query::load().all())
            .unwrap()
            .keys();

        let _ = keys.len();

        perf!("end perf_options");
    }

    // perf_many_relations
    fn perf_many_relations() {
        use test_design::db::ContainsManyRelations;

        perf!("start perf_many_relations");

        const ROWS: u16 = 500;

        // Insert rows
        for _ in 1..ROWS {
            let e = ContainsManyRelations::default();
            mimic_query!()
                .save()
                .execute(query::create().entity(e))
                .unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = mimic_query!()
            .load()
            .debug()
            .execute::<ContainsManyRelations>(query::load().all())
            .unwrap()
            .keys();

        let _ = keys.len();

        perf!("end perf_many_relations");
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

            mimic_query!()
                .save()
                .execute(query::replace().entity(e))
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

            let count = mimic_query!()
                .load()
                .execute::<Searchable>(query::load().all().search(search.clone()))
                .unwrap()
                .count();

            assert_eq!(
                count, expected,
                "search_fields test failed with criteria: {search:?}",
            );
        }
    }
}

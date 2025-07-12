use crate::{DATA_REGISTRY, INDEX_REGISTRY};
use mimic::{core::traits::Path, db::query, prelude::*};
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
            ("create_lots_blob", Self::create_lots_blob),
            ("data_key_order", Self::data_key_order),
            ("index_create_and_delete", Self::index_create_and_delete),
            ("index_option", Self::index_option),
            ("limit_query", Self::limit_query),
            ("load_one", Self::load_one),
            ("perf_options", Self::perf_options),
            ("perf_many_relations", Self::perf_many_relations),
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
        use test_design::canister::db::ContainsBlob;

        const ROWS: usize = 100;

        // Insert rows
        for _ in 0..ROWS {
            let e = ContainsBlob::default();
            db!()
                .save()
                .execute(query::create().from_entity(e))
                .unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = db!()
            .load()
            .debug()
            .execute::<ContainsBlob>(query::load().sort_field("id", SortDirection::Asc))
            .unwrap()
            .keys();

        assert!(
            keys.len() == ROWS,
            "{} rows returned, should be {ROWS}",
            keys.len()
        );

        // Verify the order
        for pair in keys.windows(2) {
            assert!(pair[0] < pair[1], "key ordering is incorrect");
        }
    }

    // create
    fn create() {
        use test_design::canister::db::CreateBasic;

        let e = CreateBasic::default();
        db!()
            .save()
            .execute(query::create().from_entity(e))
            .unwrap();

        // count keys
        let num_keys = db!()
            .load()
            .execute::<CreateBasic>(query::load())
            .unwrap()
            .count();
        assert_eq!(num_keys, 1);

        // insert another
        let e = CreateBasic::default();
        db!()
            .save()
            .execute(query::create().from_entity(e))
            .unwrap();

        // count keys

        assert_eq!(
            db!()
                .load()
                .execute::<CreateBasic>(query::load())
                .unwrap()
                .count(),
            2
        );
    }

    // create_lots
    fn create_lots() {
        use test_design::canister::db::CreateBasic;
        const ROWS: u32 = 1_000;

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBasic::default();
            db!()
                .save()
                .execute(query::create().from_entity(e))
                .unwrap();
        }

        // Retrieve the count from the store
        let count = db!()
            .load()
            .execute::<CreateBasic>(query::load())
            .unwrap()
            .count();

        // Assert that the count matches the expected number
        assert_eq!(count, ROWS, "Expected {ROWS} keys in the store");
    }

    // create_lots_blob
    fn create_lots_blob() {
        use test_design::canister::db::CreateBlob;
        const ROWS: u32 = 500;
        const BLOB_SIZE: usize = 1024 * 2;

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBlob {
                bytes: vec![0u8; BLOB_SIZE].into(),
                ..Default::default()
            };
            db!()
                .save()
                .execute(query::create().from_entity(e))
                .unwrap();
        }

        // Retrieve the count from the store
        let count = db!()
            .load()
            .execute::<CreateBlob>(query::load())
            .unwrap()
            .count();

        // Assert that the count matches the expected number
        assert_eq!(count, ROWS, "Expected {ROWS} keys in the store");
    }

    // data_key_order
    fn data_key_order() {
        use test_design::canister::db::DataKeyOrder;

        const ROWS: u16 = 1_000;

        // Insert rows
        for _ in 1..ROWS {
            let e = DataKeyOrder::default();
            db!()
                .save()
                .execute(query::create().from_entity(e))
                .unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = db!()
            .load()
            .execute::<DataKeyOrder>(query::load().sort([("id", SortDirection::Asc)]))
            .unwrap()
            .keys();

        // Verify the order
        for pair in keys.windows(2) {
            assert!(pair[0] < pair[1], "key ordering is incorrect");
        }
    }

    // index_create_and_delete
    fn index_create_and_delete() {
        use test_design::canister::index::Index;

        // Step 1: Insert entity e1 with x=1, y=10
        let e1 = Index::new(1, 10);
        db!()
            .save()
            .debug()
            .execute(query::create().from_entity(e1.clone()))
            .unwrap();

        // Step 2: Insert entity e2 with x=1 (non-unique), y=20 (unique)
        let e2 = Index::new(1, 20);
        db!()
            .save()
            .debug()
            .execute(query::create().from_entity(e2))
            .unwrap();

        // Step 3: Attempt to insert another with duplicate y=10 (should fail)
        let e3 = Index::new(2, 10);
        let result = db!()
            .save()
            .debug()
            .execute(query::create().from_entity(e3.clone()));
        assert!(result.is_err(), "expected unique index violation on y=10");

        // Step 4: Delete e1 (y=10)
        db!().delete().one::<Index>(e1.id).unwrap();

        // Step 5: Try inserting e3 again (y=10 should now be free)
        let result = db!()
            .save()
            .debug()
            .execute(query::create().from_entity(e3));
        assert!(
            result.is_ok(),
            "expected insert to succeed after y=10 was freed by delete"
        );

        // Step 6: Confirm only 2 entities remain
        let all = db!()
            .load()
            .debug()
            .execute::<Index>(query::load())
            .unwrap();

        assert_eq!(all.count(), 2);
    }

    fn index_option() {
        use test_design::canister::index::IndexUniqueOpt;

        // Insert entity with Some(10)
        let e1 = IndexUniqueOpt {
            id: Ulid::generate(),
            value: Some(10),
        };
        db!()
            .save()
            .execute(query::create().from_entity(e1.clone()))
            .unwrap();

        // Insert entity with Some(20)
        let e2 = IndexUniqueOpt {
            id: Ulid::generate(),
            value: Some(20),
        };
        db!()
            .save()
            .execute(query::create().from_entity(e2))
            .unwrap();

        // Insert entity with None (should not conflict with anything)
        let e3 = IndexUniqueOpt {
            id: Ulid::generate(),
            value: None,
        };
        db!()
            .save()
            .execute(query::create().from_entity(e3.clone()))
            .unwrap();

        // Insert duplicate Some(10) — should fail (if index is unique)
        let e4 = IndexUniqueOpt {
            id: Ulid::generate(),
            value: Some(10),
        };
        let result = db!()
            .save()
            .execute(query::create().from_entity(e4.clone()));
        assert!(
            result.is_err(),
            "Expected duplicate index error on Some(10)"
        );

        // Delete e1 (frees up Some(10))
        db!().delete().one::<IndexUniqueOpt>(e1.id).unwrap();

        // Retry insert of e4 — should now succeed
        let result = db!().save().execute(query::create().from_entity(e4));
        assert!(
            result.is_ok(),
            "Expected insert to succeed after deleting conflicting index"
        );

        // Delete e3 (value = None)
        db!().delete().one::<IndexUniqueOpt>(e3.id).unwrap();

        // Insert another entity with value = None — should be fine (no uniqueness enforced)
        let e5 = IndexUniqueOpt {
            id: Ulid::generate(),
            value: None,
        };
        db!()
            .save()
            .execute(query::create().from_entity(e5))
            .unwrap();

        // Confirm only 3 entities now exist
        let all = db!()
            .load()
            .execute::<IndexUniqueOpt>(query::load())
            .unwrap();
        assert_eq!(all.count(), 3);
    }

    fn limit_query() {
        use test_design::canister::db::Limit;

        // Insert 100 rows
        // overwrite the ulid with replace()
        for value in 1..100 {
            let e = Limit { value };
            db!()
                .save()
                .execute(query::replace().from_entity(e))
                .unwrap();
        }

        // Test various limits and offsets
        for limit in [10, 20, 50] {
            for offset in [0, 5, 10] {
                let count = db!()
                    .load()
                    .execute::<Limit>(query::load().offset(offset).limit(limit))
                    .unwrap()
                    .count();

                assert_eq!(count, limit, "{limit} not equal to {count}");
                //    if !results.is_empty() {
                //        assert_eq!(results[0].value, offset + 1);
                //    }
            }
        }
    }

    fn load_one() {
        use test_design::canister::db::CreateBasic;

        let e = CreateBasic::default();
        let id = db!()
            .save()
            .execute(query::replace().from_entity(e.clone()))
            .unwrap()
            .0
            .first()
            .unwrap()
            .key;

        let loaded = db!().load().debug().one::<CreateBasic>(id).unwrap();

        assert_eq!(loaded.id, e.id);
    }

    fn perf_options() {
        use test_design::canister::db::ContainsOpts;

        const ROWS: u16 = 500;

        // Insert rows
        for _ in 1..ROWS {
            let e = ContainsOpts::default();
            db!()
                .save()
                .execute(query::create().from_entity(e))
                .unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = db!()
            .load()
            .execute::<ContainsOpts>(query::load())
            .unwrap()
            .keys();

        let _ = keys.len();
    }

    fn perf_many_relations() {
        use test_design::canister::db::ContainsManyRelations;

        const ROWS: u16 = 500;

        // Insert rows
        for _ in 1..ROWS {
            let e = ContainsManyRelations::default();
            db!()
                .save()
                .execute(query::create().from_entity(e))
                .unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = db!()
            .load()
            .execute::<ContainsManyRelations>(query::load())
            .unwrap()
            .keys();

        let _ = keys.len();
    }
}

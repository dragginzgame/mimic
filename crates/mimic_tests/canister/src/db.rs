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
            ("load_many", Self::load_many),
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
            db!().save().create(e).unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = db!()
            .load()
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
        db!().save().create(e).unwrap();

        // count keys
        let num_keys = db!()
            .load()
            .execute::<CreateBasic>(query::load())
            .unwrap()
            .len();
        assert_eq!(num_keys, 1);

        // insert another
        let e = CreateBasic::default();
        db!().save().create(e).unwrap();

        // count keys

        assert_eq!(
            db!()
                .load()
                .execute::<CreateBasic>(query::load())
                .unwrap()
                .len(),
            2
        );
    }

    // create_lots
    fn create_lots() {
        use test_design::canister::db::CreateBasic;
        const ROWS: usize = 1_000;

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBasic::default();
            db!().save().create(e).unwrap();
        }

        // Retrieve the count from the store
        let count = db!().load().all::<CreateBasic>().unwrap().len();

        // Assert that the count matches the expected number
        assert_eq!(count, ROWS, "Expected {ROWS} keys in the store");
    }

    // create_lots_blob
    fn create_lots_blob() {
        use test_design::canister::db::CreateBlob;
        const ROWS: usize = 500;
        const BLOB_SIZE: usize = 1024 * 2;

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBlob {
                bytes: vec![0u8; BLOB_SIZE].into(),
                ..Default::default()
            };

            db!().save().create(e).unwrap();
        }

        // Retrieve the count from the store
        let count = db!()
            .load()
            .execute::<CreateBlob>(query::load())
            .unwrap()
            .len();

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
            db!().save().create(e).unwrap();
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
        let id1 = db!().save().create(e1).unwrap();

        // Step 2: Insert entity e2 with x=1 (non-unique), y=20 (unique)
        let e2 = Index::new(1, 20);
        db!().save().create(e2).unwrap();

        // Step 3: Attempt to insert another with duplicate y=10 (should fail)
        let e3 = Index::new(2, 10);
        let result = db!().save().create(e3.clone());
        assert!(result.is_err(), "expected unique index violation on y=10");

        // Step 4: Delete e1 (y=10)
        db!().delete().one::<Index>(id1).unwrap();

        // Step 5: Try inserting e3 again (y=10 should now be free)
        let result = db!().save().create(e3);
        assert!(
            result.is_ok(),
            "expected insert to succeed after y=10 was freed by delete"
        );

        // Step 6: Confirm only 2 entities remain
        let all = db!().load().all::<Index>().unwrap();

        assert_eq!(all.len(), 2);
    }

    fn index_option() {
        use test_design::canister::index::IndexUniqueOpt;

        // Insert entity with Some(10)
        let e1 = IndexUniqueOpt {
            id: Ulid::generate(),
            value: Some(10),
        };
        let id1 = db!().save().create(e1).unwrap();

        // Insert entity with Some(20)
        let e2 = IndexUniqueOpt {
            id: Ulid::generate(),
            value: Some(20),
        };
        db!().save().create(e2).unwrap();

        // Insert entity with None (should not conflict with anything)
        let e3 = IndexUniqueOpt {
            id: Ulid::generate(),
            value: None,
        };
        let id3 = db!().save().create(e3).unwrap();

        // Insert duplicate Some(10) — should fail (if index is unique)
        let e4 = IndexUniqueOpt {
            id: Ulid::generate(),
            value: Some(10),
        };
        let result = db!().save().create(e4.clone());
        assert!(
            result.is_err(),
            "Expected duplicate index error on Some(10)"
        );

        // Delete e1 (frees up Some(10))
        db!().delete().one::<IndexUniqueOpt>(id1).unwrap();

        // Retry insert of e4 — should now succeed
        let result = db!().save().create(e4);
        assert!(
            result.is_ok(),
            "Expected insert to succeed after deleting conflicting index"
        );

        // Delete e3 (value = None)
        db!().delete().one::<IndexUniqueOpt>(id3).unwrap();

        // Insert another entity with value = None — should be fine (no uniqueness enforced)
        let e5 = IndexUniqueOpt {
            id: Ulid::generate(),
            value: None,
        };
        db!().save().create(e5).unwrap();

        // Confirm only 3 entities now exist
        let all = db!().load().all::<IndexUniqueOpt>().unwrap();
        assert_eq!(all.len(), 3);
    }

    fn limit_query() {
        use test_design::canister::db::Limit;

        // Insert 100 rows
        // overwrite the ulid with replace()
        for value in 1..100 {
            let e = Limit { value };
            db!().save().replace(e).unwrap();
        }

        // Test various limits and offsets
        for limit in [10, 20, 50] {
            for offset in [0, 5, 10] {
                let count = db!()
                    .load()
                    .execute::<Limit>(query::load().offset(offset).limit(limit))
                    .unwrap()
                    .len();

                assert_eq!(count, limit as usize, "{limit} not equal to {count}");
                //    if !results.is_empty() {
                //        assert_eq!(results[0].value, offset + 1);
                //    }
            }
        }
    }

    fn load_one() {
        use test_design::canister::db::CreateBasic;

        let id = db!().save().create(CreateBasic::default()).unwrap();

        let loaded = db!().load().one::<CreateBasic>(id).unwrap();

        assert_eq!(loaded.key(), id);
    }

    fn load_many() {
        use test_design::canister::db::CreateBasic;

        let key1 = db!().save().create(CreateBasic::default()).unwrap();
        let key2 = db!().save().create(CreateBasic::default()).unwrap();
        let key3 = db!().save().create(CreateBasic::default()).unwrap();

        // Pass a slice of IDs
        let many_keys = vec![key1, key2, key3];
        let loaded = db!().load().many::<CreateBasic, _>(&many_keys).unwrap();

        // Assert correct count
        assert_eq!(loaded.len(), 3);

        // Optionally assert that each loaded item has a matching ID
        for key in loaded.keys() {
            assert!(many_keys.contains(&key));
        }
    }

    fn perf_options() {
        use test_design::canister::db::ContainsOpts;

        const ROWS: u16 = 500;

        // Insert rows
        for _ in 1..ROWS {
            let e = ContainsOpts::default();
            db!().save().create(e).unwrap();
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
            db!().save().create(e).unwrap();
        }

        // Retrieve rows in B-Tree order
        let rows = db!().load().all::<ContainsManyRelations>().unwrap();

        let _ = rows.len();
    }
}

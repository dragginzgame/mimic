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
            ("create_and_delete_index", Self::create_and_delete_index),
            ("create_lots", Self::create_lots),
            ("create_lots_blob", Self::create_lots_blob),
            ("data_key_order", Self::data_key_order),
            ("limit_query", Self::limit_query),
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
            db!().save().execute(query::create().entity(e)).unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = db!()
            .load()
            .debug()
            .execute::<ContainsBlob>(query::load().all().sort_field("id", SortDirection::Asc))
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
        db!().save().execute(query::create().entity(e)).unwrap();

        // count keys
        let num_keys = db!()
            .load()
            .execute::<CreateBasic>(query::load().all())
            .unwrap()
            .count();
        assert_eq!(num_keys, 1);

        // insert another
        let e = CreateBasic::default();
        db!().save().execute(query::create().entity(e)).unwrap();

        // count keys

        assert_eq!(
            db!()
                .load()
                .execute::<CreateBasic>(query::load().all())
                .unwrap()
                .count(),
            2
        );
    }

    // create_and_delete_index
    fn create_and_delete_index() {
        use test_design::canister::index::Index;

        // Step 1: Insert entity e1 with x=1, y=10
        let e1 = Index::new(1, 10);
        db!()
            .save()
            .debug()
            .execute(query::create().entity(e1.clone()))
            .unwrap();

        // Step 2: Insert entity e2 with x=1 (non-unique), y=20 (unique)
        let e2 = Index::new(1, 20);
        db!()
            .save()
            .debug()
            .execute(query::create().entity(e2))
            .unwrap();

        // Step 3: Attempt to insert another with duplicate y=10 (should fail)
        let e3 = Index::new(2, 10);
        let result = db!()
            .save()
            .debug()
            .execute(query::create().entity(e3.clone()));
        assert!(result.is_err(), "expected unique index violation on y=10");

        // Step 4: Delete e1 (y=10)
        db!()
            .delete()
            .debug()
            .execute::<Index>(query::delete().one(vec![e1.id]))
            .unwrap();

        // Step 5: Try inserting e3 again (y=10 should now be free)
        let result = db!().save().debug().execute(query::create().entity(e3));
        assert!(
            result.is_ok(),
            "expected insert to succeed after y=10 was freed by delete"
        );

        // Step 6: Confirm only 2 entities remain
        let all = db!()
            .load()
            .debug()
            .execute::<Index>(query::load().all())
            .unwrap();

        assert_eq!(all.count(), 2);
    }

    // create_lots
    fn create_lots() {
        use test_design::canister::db::CreateBasic;
        const ROWS: u32 = 1_000;

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBasic::default();
            db!().save().execute(query::create().entity(e)).unwrap();
        }

        // Retrieve the count from the store
        let count = db!()
            .load()
            .execute::<CreateBasic>(query::load().all())
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
            db!().save().execute(query::create().entity(e)).unwrap();
        }

        // Retrieve the count from the store
        let count = db!()
            .load()
            .execute::<CreateBlob>(query::load().all())
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
            db!().save().execute(query::create().entity(e)).unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = db!()
            .load()
            .execute::<DataKeyOrder>(query::load().all().sort([("id", SortDirection::Asc)]))
            .unwrap()
            .keys();

        // Verify the order
        for pair in keys.windows(2) {
            assert!(pair[0] < pair[1], "key ordering is incorrect");
        }
    }

    // limit_query
    fn limit_query() {
        use test_design::canister::db::Limit;

        // Insert 100 rows
        // overwrite the ulid with replace()
        for value in 1..100 {
            let e = Limit { value };
            db!().save().execute(query::replace().entity(e)).unwrap();
        }

        // Test various limits and offsets
        for limit in [10, 20, 50] {
            for offset in [0, 5, 10] {
                let count = db!()
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

    // perf_options
    fn perf_options() {
        use test_design::canister::db::ContainsOpts;

        const ROWS: u16 = 500;

        // Insert rows
        for _ in 1..ROWS {
            let e = ContainsOpts::default();
            db!().save().execute(query::create().entity(e)).unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = db!()
            .load()
            .debug()
            .execute::<ContainsOpts>(query::load().all())
            .unwrap()
            .keys();

        let _ = keys.len();
    }

    // perf_many_relations
    fn perf_many_relations() {
        use test_design::canister::db::ContainsManyRelations;

        const ROWS: u16 = 500;

        // Insert rows
        for _ in 1..ROWS {
            let e = ContainsManyRelations::default();
            db!().save().execute(query::create().entity(e)).unwrap();
        }

        // Retrieve rows in B-Tree order
        let keys = db!()
            .load()
            .debug()
            .execute::<ContainsManyRelations>(query::load().all())
            .unwrap()
            .keys();

        let _ = keys.len();
    }
}

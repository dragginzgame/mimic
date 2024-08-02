// darling(default) generates these warnings
#![allow(clippy::unused_self)]

use base::types::{self, Ulid};
use db::{DataKey, Db};
use db_query::types::Order;
use orm::traits::Path;

///
/// DbTester
///

const STORE: &str = ::base::canister::test::store::Data::PATH;

pub struct DbTester<'a> {
    db: &'a Db,
}

impl<'a> DbTester<'a> {
    #[must_use]
    pub const fn new(db: &'a Db) -> Self {
        DbTester { db }
    }

    // test
    // best if these are kept in code order so we can see where it failed
    pub fn test(&self) {
        self.entity_with_map();

        self.data_key_order();

        self.clear();
        self.create();
        self.create_lots();

        self.filter_query();
        self.limit_query();

        self.missing_field();
    }

    //
    // TESTS
    //

    // entity_with_map
    fn entity_with_map(&self) {
        use types::test::has_map::HasMap;

        // create with map data
        let mut e = HasMap::default();
        e.map_int_string.insert(3, "value".to_string());
        e.map_int_string.insert(4, "value".to_string());
        db_query::create(self.db)
            .from_entity(e)
            .unwrap()
            .entity::<HasMap>()
            .unwrap();

        // load all keys
        let entities = db_query::load::<HasMap>(self.db)
            .only()
            .execute_dyn()
            .unwrap()
            .keys();

        assert!(entities.count() == 1);
    }

    // data_key_order
    fn data_key_order(&self) {
        use types::test::store::SortKeyOrder;

        const ROWS: u16 = 1_000;

        // clear
        let _ = self.db.with_store_mut(STORE, |store| {
            store.clear();
            Ok(())
        });

        // Insert rows
        for _ in 1..ROWS {
            let e = SortKeyOrder::default();
            db_query::create(self.db).from_entity(e).unwrap();
        }

        // Retrieve rows in B-Tree order
        let rows: Vec<DataKey> = db_query::load::<SortKeyOrder>(self.db)
            .all()
            .order(Order::from(vec!["id"]))
            .execute()
            .unwrap()
            .keys()
            .collect();

        // Verify the order
        for i in 0..(rows.len() - 1) {
            assert!(
                rows[i] < rows[i + 1],
                "Row ordering is incorrect at index {i}"
            );
        }
    }

    // clear
    fn clear(&self) {
        use types::test::store::CreateBasic;

        // Insert rows
        for _ in 0..100 {
            let e = CreateBasic::default();
            db_query::create(self.db).from_entity(e).unwrap();
        }

        // clear
        let _ = self.db.with_store_mut(STORE, |store| {
            store.clear();
            Ok(())
        });

        // Retrieve the count of keys (or entities) from the store
        let count = db_query::load::<CreateBasic>(self.db)
            .all()
            .execute_dyn()
            .unwrap()
            .count();

        assert_eq!(count, 0, "Expected 0 keys in the store");
    }

    // create
    fn create(&self) {
        use types::test::store::CreateBasic;

        // clear
        let _ = self.db.with_store_mut(STORE, |store| {
            store.clear();
            Ok(())
        });

        let e = CreateBasic::default();
        db_query::create(self.db).from_entity(e).unwrap();

        // count keys
        assert_eq!(
            db_query::load::<CreateBasic>(self.db)
                .all()
                .execute_dyn()
                .unwrap()
                .keys()
                .count(),
            1
        );

        // insert another
        let e = CreateBasic::default();
        db_query::create(self.db).from_entity(e).unwrap();

        // count keys
        assert_eq!(
            db_query::load::<CreateBasic>(self.db)
                .all()
                .execute_dyn()
                .unwrap()
                .keys()
                .count(),
            2
        );
    }

    // create_lots
    fn create_lots(&self) {
        use types::test::store::CreateBasic;
        const ROWS: usize = 5_000;

        // clear
        let _ = self.db.with_store_mut(STORE, |store| {
            store.clear();
            Ok(())
        });

        // Insert 10,000 rows
        for _ in 0..ROWS {
            let e = CreateBasic::default();
            db_query::create(self.db).from_entity(e).unwrap();
        }

        // Retrieve the count from the store
        let count = db_query::load::<CreateBasic>(self.db)
            .all()
            .execute_dyn()
            .unwrap()
            .count();

        // Assert that the count matches the expected number
        assert_eq!(count, ROWS, "Expected {ROWS} keys in the store");
    }

    // filter_query
    fn filter_query(&self) {
        use types::test::store::Filterable;

        // clear
        let _ = self.db.with_store_mut(STORE, |store| {
            store.clear();
            Ok(())
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
                id: Ulid::from_string(id).unwrap(),
                name: name.into(),
                description: description.into(),
            };
            db_query::replace(self.db).from_entity(e).unwrap();
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
            let count = db_query::load::<Filterable>(self.db)
                .all()
                .filter(search)
                .execute()
                .unwrap()
                .keys()
                .count();

            assert_eq!(
                count, expected_count,
                "Test for string '{search}' in [Filter] failed",
            );
        }
    }

    // limit_query
    fn limit_query(&self) {
        use types::test::store::Limit;

        // clear
        let _ = self.db.with_store_mut(STORE, |store| {
            store.clear();
            Ok(())
        });

        // Insert 100 rows
        // overwrite the ulid with replace()
        for value in 1..100 {
            let e = Limit { value };
            db_query::replace(self.db).from_entity(e).unwrap();
        }

        // Test various limits and offsets
        for limit in [10, 20, 50] {
            for offset in [0, 5, 10] {
                let results = db_query::load::<Limit>(self.db)
                    .all()
                    .offset(offset)
                    .limit(limit)
                    .execute_dyn()
                    .unwrap()
                    .keys();

                assert_eq!(results.count(), limit as usize);
                //    if !results.is_empty() {
                //        assert_eq!(results[0].value, offset + 1);
                //    }
            }
        }
    }

    // missing_field
    fn missing_field(&self) {
        use types::test::store::{MissingFieldLarge, MissingFieldSmall};

        let small = MissingFieldSmall {
            a: Ulid::generate(),
            b: Ulid::generate(),
        };

        // move from small to large
        let bytes = orm::serialize(&small).unwrap();
        let large = orm::deserialize::<MissingFieldLarge>(&bytes).unwrap();

        assert!(!large.a.is_nil());
        assert!(!large.b.is_nil());
        assert!(large.c.is_nil());
    }
}

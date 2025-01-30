use mimic::{
    db::{store::Store, types::DataKey, Db},
    ic::structures::{
        memory_manager::{MemoryId, MemoryManager},
        DefaultMemoryImpl,
    },
    orm::prelude::*,
    query::types::Order,
};
use std::cell::RefCell;

thread_local! {
    pub static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static STORE: RefCell<Store> = RefCell::new(
        Store::init(
            MEMORY_MANAGER.with(|mm| mm.borrow().get(
                MemoryId::new(1)
            ))
        )
    );

    static DB: RefCell<Db> = RefCell::new({
        let mut db = Db::new();
        db.insert("store", &STORE);

        db
    });
}

///
/// DbTester
///

pub struct DbTester {}

impl DbTester {
    // test
    // best if these are kept in code order so we can see where it failed
    pub fn test() {
        Self::entity_with_map();

        Self::data_key_order();

        Self::clear();
        Self::create();
        Self::create_lots();

        Self::filter_query();
        Self::limit_query();

        Self::missing_field();
    }

    //
    // TESTS
    //

    // entity_with_map
    fn entity_with_map() {
        use test_schema::map::HasMap;

        // create with map data
        let mut e = HasMap::default();
        e.map_int_string.push((3, "value".to_string()));
        e.map_int_string.push((4, "value".to_string()));
        query::create_entity::<HasMap>()
            .from_entity(e)
            .execute(&STORE)
            .unwrap();

        // load all keys
        let entities = query::load_entity::<HasMap>()
            .only()
            .execute(&STORE)
            .unwrap()
            .keys();

        assert!(entities.count() == 1);
    }

    // data_key_order
    fn data_key_order() {
        use test_schema::db::SortKeyOrder;

        const ROWS: u16 = 1_000;

        // clear
        STORE.with_borrow_mut(|store| {
            store.clear();
        });

        // Insert rows
        for _ in 1..ROWS {
            let e = SortKeyOrder::default();
            query::create().from_entity(e).execute(&STORE).unwrap();
        }

        // Retrieve rows in B-Tree order
        let rows: Vec<DataKey> = query::load_entity::<SortKeyOrder>()
            .all()
            .order(Order::from(vec!["id"]))
            .execute(&STORE)
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
    fn clear() {
        use test_schema::db::CreateBasic;

        // Insert rows
        for _ in 0..100 {
            let e = CreateBasic::default();
            query::create().from_entity(e).execute(&STORE).unwrap();
        }

        // clear
        STORE.with_borrow_mut(|store| {
            store.clear();
        });
        // Retrieve the count of keys (or entities) from the store
        let count = query::load_entity::<CreateBasic>()
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
        STORE.with_borrow_mut(|store| {
            store.clear();
        });

        let e = CreateBasic::default();
        query::create().from_entity(e).execute(&STORE).unwrap();

        // count keys
        assert_eq!(
            query::load_entity::<CreateBasic>()
                .all()
                .execute(&STORE)
                .unwrap()
                .keys()
                .count(),
            1
        );

        // insert another
        let e = CreateBasic::default();
        query::create().from_entity(e).execute(&STORE).unwrap();

        // count keys
        assert_eq!(
            query::load_entity::<CreateBasic>()
                .all()
                .execute(&STORE)
                .unwrap()
                .keys()
                .count(),
            2
        );
    }

    // create_lots
    fn create_lots() {
        use test_schema::db::CreateBasic;
        const ROWS: usize = 1_000;

        // clear
        STORE.with_borrow_mut(|store| {
            store.clear();
        });

        // insert rows
        for _ in 0..ROWS {
            let e = CreateBasic::default();
            query::create().from_entity(e).execute(&STORE).unwrap();
        }

        // Retrieve the count from the store
        let count = query::load_entity::<CreateBasic>()
            .all()
            .execute(&STORE)
            .unwrap()
            .count();

        // Assert that the count matches the expected number
        assert_eq!(count, ROWS, "Expected {ROWS} keys in the store");
    }

    // filter_query
    fn filter_query() {
        use test_schema::db::Filterable;

        // clear
        STORE.with_borrow_mut(|store| {
            store.clear();
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
            query::replace().from_entity(e).execute(&STORE).unwrap();
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
            let count = query::load_entity::<Filterable>()
                .all()
                .filter_all(search)
                .execute(&STORE)
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
    fn limit_query() {
        use test_schema::db::Limit;

        // clear
        STORE.with_borrow_mut(|store| {
            store.clear();
        });

        // Insert 100 rows
        // overwrite the ulid with replace()
        for value in 1..100 {
            let e = Limit { value };
            query::replace()
                .debug()
                .from_entity(e)
                .execute(&STORE)
                .unwrap();
        }

        // Test various limits and offsets
        for limit in [10, 20, 50] {
            for offset in [0, 5, 10] {
                let results = query::load_entity::<Limit>()
                    .all()
                    .offset(offset)
                    .limit(limit)
                    .execute(&STORE)
                    .unwrap()
                    .keys();

                let count = results.count();
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
        let bytes = mimic::orm::serialize(&small).unwrap();
        let large = mimic::orm::deserialize::<MissingFieldLarge>(&bytes).unwrap();

        assert!(!large.a_id.is_nil());
        assert!(!large.b_id.is_nil());
        assert!(large.c_id.is_nil());
    }
}

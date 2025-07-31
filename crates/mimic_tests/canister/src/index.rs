use mimic::{
    core::{
        traits::{EntityKind, Path},
        types::Principal,
    },
    db::query::{self, LoadQuery, QueryPlan, QueryPlanner},
    prelude::*,
};
use test_design::{
    canister::index::{Indexable, NotIndexable},
    schema::TestDataStore,
};

pub struct IndexTester;

impl IndexTester {
    pub fn test() {
        let tests: Vec<(&str, fn())> = vec![
            ("index_on_principal", Self::index_on_principal),
            ("index_on_principal_ulid", Self::index_on_principal_ulid),
            ("index_on_all_fields", Self::index_on_all_fields),
            ("fallback_to_range", Self::fallback_to_range),
            ("negative_index_miss", Self::negative_index_miss),
        ];

        for (name, test_fn) in tests {
            println!("Clearing DB...");
            crate::DATA_REGISTRY.with(|reg| {
                reg.with_store_mut(TestDataStore::PATH, |store| store.clear())
                    .ok();
            });

            println!("Running test: {name}");
            test_fn();
        }
    }

    fn index_on_principal() {
        let pid = Principal::from_slice(&[1; 29]);
        let query = query::load().filter(|f| f.filter("pid", Cmp::Eq, pid));

        assert_uses_index::<Indexable>(&query);

        let results = db!().load().execute::<Indexable>(query).unwrap().entities();

        assert!(results.iter().all(|e| e.pid == pid));
    }

    fn index_on_principal_ulid() {
        let pid = Principal::from_slice(&[1; 29]);
        let ulid = Ulid::from_u128(1);
        let query =
            query::load().filter(|f| f.filter("pid", Cmp::Eq, pid).filter("ulid", Cmp::Eq, ulid));

        assert_uses_index::<Indexable>(&query);

        let results = db!().load().execute::<Indexable>(query).unwrap().entities();

        assert!(results.iter().all(|e| e.pid == pid && e.ulid == ulid));
    }

    fn index_on_all_fields() {
        let pid = Principal::from_slice(&[1; 29]);
        let ulid = Ulid::from_u128(1);
        let score = 10u32;

        let query = query::load().filter(|f| {
            f.filter("pid", Cmp::Eq, pid)
                .filter("ulid", Cmp::Eq, ulid)
                .filter("score", Cmp::Eq, score)
        });

        assert_uses_index::<Indexable>(&query);

        let results = db!().load().execute::<Indexable>(query).unwrap().entities();

        assert!(
            results
                .iter()
                .all(|e| e.pid == pid && e.ulid == ulid && e.score == score)
        );
    }

    fn fallback_to_range() {
        let query = query::load().filter(|f| f.filter("score", Cmp::Gt, 50));

        let planner = QueryPlanner::new(query.filter.as_ref());
        let plan = planner.plan::<NotIndexable>();

        match plan {
            QueryPlan::Range(_, _) => println!("✅ Fallback to range plan"),
            _ => panic!("❌ Expected fallback Range plan, got: {plan:?}"),
        }
    }

    fn negative_index_miss() {
        let query =
            query::load().filter(|f| f.filter("pid", Cmp::Eq, Principal::from_slice(&[99; 29])));
        assert_uses_index::<Indexable>(&query);

        let results = db!().load().execute::<Indexable>(query).unwrap().entities();
        assert!(
            results.is_empty(),
            "Expected no results from unmatched index lookup"
        );
    }
}

fn assert_uses_index<E: EntityKind>(query: &LoadQuery) {
    let planner = QueryPlanner::new(query.filter.as_ref());
    let plan = planner.plan::<E>();

    match plan {
        QueryPlan::Index { .. } => println!("✅ Used index"),
        _ => panic!("❌ Expected index-based query plan, got: {plan:?}"),
    }
}

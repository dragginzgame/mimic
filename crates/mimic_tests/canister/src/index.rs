use mimic::{core::traits::Path, db::query, prelude::*};
use test_design::{canister::index::Indexable, schema::TestStore};

///
/// IndexTester
///

pub struct IndexTester;

impl IndexTester {
    pub fn test() {
        let tests: Vec<(&str, fn())> = vec![("index_on_group", Self::index_on_group)];

        for (name, test_fn) in tests {
            println!("Clearing DB...");
            crate::DATA_REGISTRY.with(|reg| {
                reg.with_store_mut(TestStore::PATH, |store| store.clear())
                    .ok();
            });

            println!("Running test: {name}");
            test_fn();
        }
    }

    fn index_on_group() {
        let query = query::load().with_filter(|f| f.and("group", Cmp::Eq, "alpha"));

        let results = db!().load().execute::<Indexable>(query).unwrap().entities();

        assert!(results.iter().all(|e| e.group == "alpha"));
        println!("✅ Passed: index_on_group");
    }
}

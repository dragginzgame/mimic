use mimic::{
    core::traits::Path,
    db::query::{self, FilterClause, FilterExpr},
    prelude::*,
};
use test_design::{canister::filter::Filterable, schema::TestStore};

///
/// FilterTester
///

pub struct FilterTester {}

impl FilterTester {
    // test
    // best if these are kept in code order so we can see where it failed
    pub fn test() {
        let tests: Vec<(&str, fn())> = vec![
            ("filter_eq_string", Self::filter_eq_string),
            ("filter_eq_bool", Self::filter_eq_bool),
            ("filter_gt_score", Self::filter_gt_score),
            ("filter_le_level", Self::filter_le_level),
            ("filter_ne_category", Self::filter_ne_category),
            ("filter_and_group", Self::filter_and_group),
            ("filter_or_group", Self::filter_or_group),
            ("filter_nested_groups", Self::filter_nested_groups),
            ("filter_startswith_name", Self::filter_startswith_name),
            ("filter_not_clause", Self::filter_not_clause),
            ("filter_true_short_circuit", Self::filter_true_short_circuit),
            (
                "filter_false_short_circuit",
                Self::filter_false_short_circuit,
            ),
            ("filter_empty_result", Self::filter_empty_result),
            ("filter_in_category", Self::filter_in_category),
            ("filter_allin_tags", Self::filter_allin_tags),
            ("filter_anyin_tags", Self::filter_anyin_tags),
            ("filter_contains_tag", Self::filter_contains_tag),
        ];

        for (name, test_fn) in tests {
            println!("clearing db");
            crate::DATA_REGISTRY.with(|reg| {
                reg.with_store_mut(TestStore::PATH, |store| store.clear())
                    .ok();
            });

            println!("Running test: {name}");
            test_fn();
        }
    }

    // filter
    fn filter_eq_string() {
        let query = query::load().with_filter(|f| f.filter("category", Cmp::Eq, "A"));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert!(results.iter().all(|e| e.category == "A"));
    }

    fn filter_eq_bool() {
        let query = query::load().with_filter(|f| f.filter("active", Cmp::Eq, true));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert!(results.iter().all(|e| e.active));
    }

    fn filter_gt_score() {
        let query = query::load().with_filter(|f| f.filter("score", Cmp::Gt, 80.0));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert!(results.iter().all(|e| e.score > 80.0));
    }

    fn filter_le_level() {
        let query = query::load().with_filter(|f| f.filter("level", Cmp::Lte, 3));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert!(results.iter().all(|e| e.level <= 3));
    }

    fn filter_ne_category() {
        let query = query::load().with_filter(|f| f.filter("category", Cmp::Ne, "B"));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert!(results.iter().all(|e| e.category != "B"));
    }

    fn filter_and_group() {
        let query = query::load().with_filter(|f| {
            f.filter_group(|b| {
                b.filter("score", Cmp::Gte, 60.0)
                    .filter("level", Cmp::Gte, 2)
            })
        });

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert!(results.iter().all(|e| e.score >= 60.0 && e.level >= 2));
    }

    fn filter_or_group() {
        let query = query::load().with_filter(|f| {
            f.or_filter_group(|b| {
                b.filter("category", Cmp::Eq, "A")
                    .filter("category", Cmp::Eq, "C")
            })
        });

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert!(
            results
                .iter()
                .all(|e| e.category == "A" || e.category == "C")
        );
    }

    fn filter_nested_groups() {
        let query = query::load().with_filter(|f| {
            f.filter("active", Cmp::Eq, true).or_filter_group(|b| {
                b.filter_group(|b| b.filter("score", Cmp::Lt, 40.0))
                    .or_filter("offset", Cmp::Lt, 0)
            })
        });

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert!(!results.is_empty());
    }

    fn filter_startswith_name() {
        let query = query::load().with_filter(|f| f.filter("name", Cmp::StartsWith, "A"));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert!(results.iter().all(|e| e.name.starts_with('A')));
    }

    fn filter_not_clause() {
        let expr = FilterExpr::Clause(FilterClause::new("category", Cmp::Eq, "C")).not();
        let query = query::load().set_filter(expr);

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert!(results.iter().all(|e| e.category != "C"));
    }

    fn filter_true_short_circuit() {
        let query = query::load().set_filter(FilterExpr::True);
        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert_eq!(results.len(), 10); // all fixtures
    }

    fn filter_false_short_circuit() {
        let results = db!()
            .load()
            .execute::<Filterable>(query::load().set_filter(FilterExpr::False))
            .unwrap()
            .entities();

        assert_eq!(results.len(), 0);
    }

    fn filter_empty_result() {
        let query = query::load().with_filter(|f| f.filter("category", Cmp::Eq, "Nonexistent"));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert_eq!(results.len(), 0);
    }

    fn filter_in_category() {
        let query =
            query::load().with_filter(|f| f.filter("category", Cmp::In, Value::list(&["A", "C"])));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert!(
            !results.is_empty(),
            "Expected results for IN filter, got none"
        );

        assert!(
            results
                .iter()
                .all(|e| e.category == "A" || e.category == "C")
        );
    }

    fn filter_allin_tags() {
        let query = query::load()
            .with_filter(|f| f.filter("tags", Cmp::AllIn, Value::list(&["blue", "green"])));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert!(
            !results.is_empty(),
            "Expected results for ALL IN filter, got none"
        );

        assert!(results.iter().all(|e| {
            e.tags.contains(&"blue".to_string()) && e.tags.contains(&"green".to_string())
        }));
    }

    fn filter_anyin_tags() {
        let query = query::load()
            .with_filter(|f| f.filter("tags", Cmp::AnyIn, Value::list(&["blue", "green"])));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert!(
            !results.is_empty(),
            "Expected results for ANY IN filter, got none"
        );

        for e in &results {
            assert!(
                e.tags.contains(&"blue".to_string()) || e.tags.contains(&"green".to_string()),
                "Entity {:?} did not match ANY IN condition",
                e.name
            );
        }
    }

    fn filter_contains_tag() {
        let query = query::load().with_filter(|f| f.filter("tags", Cmp::Contains, "green"));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert!(
            results
                .iter()
                .all(|e| e.tags.contains(&"green".to_string()))
        );
    }
}

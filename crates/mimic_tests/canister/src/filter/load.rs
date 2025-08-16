use mimic::{
    core::types::{Decimal, Principal},
    prelude::*,
};
use test_design::canister::filter::{Filterable, FilterableOpt};

///
/// LoadFilterTester
///

pub struct LoadFilterTester {}

impl LoadFilterTester {
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
            ("filter_always", Self::filter_always),
            ("filter_never", Self::filter_never),
            ("filter_empty_result", Self::filter_empty_result),
            ("filter_in_category", Self::filter_in_category),
            ("filter_allin_tags", Self::filter_allin_tags),
            ("filter_anyin_tags", Self::filter_anyin_tags),
            ("filter_eq_principal", Self::filter_eq_principal),
            ("filter_contains_tag", Self::filter_contains_tag),
            // opt
            ("filter_opt_eq_name", Self::filter_opt_eq_name),
            ("filter_opt_lt_level", Self::filter_opt_lt_level),
            ("filter_opt_is_none_name", Self::filter_opt_is_none_name),
            ("filter_opt_ne_pid_null", Self::filter_opt_ne_pid_null),
        ];

        // insert data
        Self::insert();
        Self::insert_opt();

        for (name, test_fn) in tests {
            println!("Running test: {name}");
            test_fn();
        }
    }

    // insert
    fn insert() {
        let fixtures = [
            ("Alpha", "A", true, 87.2, 1, -10, vec!["red", "blue"], 1),
            ("Beta", "B", false, 65.1, 2, 0, vec!["green"], 2),
            ("Gamma", "C", true, 92.5, 3, 10, vec!["red", "yellow"], 3),
            ("Delta", "B", false, 15.3, 2, 5, vec![], 4),
            ("Epsilon", "A", true, 75.0, 4, -5, vec!["green", "blue"], 5),
            ("Zeta", "C", false, 88.8, 5, 15, vec!["purple"], 6),
            ("Eta", "B", true, 30.5, 1, 8, vec!["red"], 7),
            ("Theta", "A", true, 99.9, 6, -20, vec!["blue", "green"], 8),
            ("Iota", "C", false, 42.0, 3, 0, vec!["yellow", "red"], 9),
            ("Kappa", "B", true, 50.0, 2, 3, vec!["green", "blue"], 10),
        ];

        for (name, category, active, score, level, offset, tags, pid_index) in fixtures {
            EntityService::save_fixture(
                db!(),
                Filterable {
                    name: name.into(),
                    category: category.into(),
                    active,
                    score: Decimal::from(score),
                    level,
                    offset,
                    tags: tags.iter().map(ToString::to_string).collect(),
                    pid: Principal::dummy(pid_index),
                    ..Default::default()
                },
            );
        }
    }

    fn insert_opt() {
        let fixtures = [
            (Some("Alice"), Some(1), Some(-10), Some(Principal::dummy(1))),
            (Some("Bob"), Some(2), Some(0), Some(Principal::dummy(2))),
            (None, Some(3), None, Some(Principal::dummy(3))),
            (Some("Charlie"), None, Some(5), None),
            (None, None, None, None),
        ];

        for (name, level, offset, pid) in fixtures {
            EntityService::save_fixture(
                db!(),
                FilterableOpt {
                    name: name.map(str::to_string),
                    level,
                    offset,
                    pid,
                    ..Default::default()
                },
            );
        }
    }

    ///
    /// NORMAL (Filterable)
    ///

    fn filter_eq_string() {
        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.eq("category", "A"))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.category == "A"));
        assert_eq!(results.len(), 3);
    }

    fn filter_eq_bool() {
        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.eq("active", true))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.active));
        assert_eq!(results.len(), 6);
    }

    fn filter_gt_score() {
        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.gt("score", Decimal::from(80.0)))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.score > Decimal::from(80.0)));
        assert_eq!(results.len(), 4);
    }

    fn filter_le_level() {
        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.lte("level", 3))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.level <= 3));
        assert_eq!(results.len(), 7);
    }

    fn filter_ne_category() {
        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.ne("category", "B"))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.category != "B"));
        assert_eq!(results.len(), 6);
    }

    fn filter_and_group() {
        let query =
            query::load().filter(|f| f.gte("score", Decimal::from(60.0)) & f.gte("level", 2));

        let results = db!()
            .load::<Filterable>()
            .execute(&query)
            .unwrap()
            .entities();

        assert!(
            results
                .iter()
                .all(|e| e.score >= Decimal::from(60.0) && e.level >= 2)
        );
        assert_eq!(results.len(), 5);
    }

    fn filter_or_group() {
        let query = query::load().filter(|f| f.eq("category", "A") | f.eq("category", "C"));

        let results = db!()
            .load::<Filterable>()
            .execute(&query)
            .unwrap()
            .entities();

        assert!(
            results
                .iter()
                .all(|e| e.category == "A" || e.category == "C")
        );
        assert_eq!(results.len(), 6);
    }

    fn filter_nested_groups() {
        let query = query::load().filter(|f| {
            f.eq("active", true) | f.lt("score", Decimal::from(40.0)) | f.lt("offset", 0)
        });

        let results = db!()
            .load::<Filterable>()
            .execute(&query)
            .unwrap()
            .entities();

        assert_eq!(results.len(), 7);
    }

    fn filter_startswith_name() {
        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.starts_with("name", "A"))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.name.starts_with('A')));
        assert_eq!(results.len(), 1);
    }

    fn filter_not_clause() {
        let query = query::load().filter(|f| f.ne("category", "C"));

        let results = db!()
            .load::<Filterable>()
            .execute(&query)
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.category != "C"));
        assert_eq!(results.len(), 7);
    }

    fn filter_always() {
        let results = db!()
            .load::<Filterable>()
            .filter(FilterDsl::always)
            .unwrap()
            .entities();

        assert_eq!(results.len(), 10); // all fixtures
    }

    fn filter_never() {
        let results = db!()
            .load::<Filterable>()
            .filter(FilterDsl::never)
            .unwrap()
            .entities();

        assert_eq!(results.len(), 0);
    }

    fn filter_empty_result() {
        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.eq("category", "Nonexistent"))
            .unwrap()
            .entities();

        assert_eq!(results.len(), 0);
    }

    fn filter_in_category() {
        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.in_iter("category", ["A", "C"]))
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
        assert_eq!(results.len(), 6);
    }

    fn filter_allin_tags() {
        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.all_in("tags", ["blue", "green"]))
            .unwrap()
            .entities();

        assert!(
            !results.is_empty(),
            "Expected results for ALL IN filter, got none"
        );

        assert!(results.iter().all(|e| {
            e.tags.contains(&"blue".to_string()) && e.tags.contains(&"green".to_string())
        }));

        assert_eq!(results.len(), 3);
    }

    fn filter_anyin_tags() {
        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.any_in("tags", ["blue", "green"]))
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

        assert_eq!(results.len(), 5);
    }

    fn filter_eq_principal() {
        // Use dummy principal that matches the one used in fixtures
        let expected = Principal::dummy(1);

        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.eq("pid", expected))
            .unwrap()
            .entities();

        assert!(
            !results.is_empty(),
            "Expected at least one entity with matching principal"
        );
        assert!(
            results.iter().all(|e| e.pid == expected),
            "All results should have matching principal"
        );
        assert_eq!(results.len(), 1);
    }

    fn filter_contains_tag() {
        let results = db!()
            .load::<Filterable>()
            .filter(|f| f.contains("tags", "green"))
            .unwrap()
            .entities();

        assert!(
            results
                .iter()
                .all(|e| e.tags.contains(&"green".to_string()))
        );
        assert_eq!(results.len(), 4);
    }

    ///
    /// OPTIONAL (FilterableOpt)
    ///

    fn filter_opt_eq_name() {
        let results = db!()
            .load::<FilterableOpt>()
            .filter(|f| f.eq("name", "Alice"))
            .unwrap()
            .entities();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name.as_deref(), Some("Alice"));
    }

    fn filter_opt_is_none_name() {
        let results = db!()
            .load::<FilterableOpt>()
            .filter(|f| f.eq("name", Value::None))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.name.is_none()));
        assert_eq!(results.len(), 2); // Charlie and None
    }

    fn filter_opt_lt_level() {
        let results = db!()
            .load::<FilterableOpt>()
            .filter(|f| f.lt("level", 3))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.level.unwrap_or(255) < 3));
        assert_eq!(results.len(), 2); // Alice (1), Bob (2)
    }

    fn filter_opt_ne_pid_null() {
        let results = db!()
            .load::<FilterableOpt>()
            .filter(|f| f.ne("pid", Value::None))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.pid.is_some()));
        assert_eq!(results.len(), 3);
    }
}

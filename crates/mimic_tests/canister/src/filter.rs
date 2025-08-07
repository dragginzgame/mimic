use mimic::{
    db::query::{self, FilterClause, FilterExpr},
    prelude::*,
};
use test_design::canister::filter::Filterable;

///
/// FilterTester
///

pub struct FilterTester {}

impl FilterTester {
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
            ("filter_eq_principal", Self::filter_eq_principal),
            ("filter_contains_tag", Self::filter_contains_tag),
        ];

        // insert data
        Self::insert();

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
                    score,
                    level,
                    offset,
                    tags: tags.iter().map(ToString::to_string).collect(),
                    pid: Filterable::dummy_principal(pid_index),
                    ..Default::default()
                },
            );
        }
    }

    fn filter_eq_string() {
        let query = query::load().filter(|f| f.filter("category", Cmp::Eq, "A"));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.category == "A"));
        assert_eq!(results.len(), 3);
    }

    fn filter_eq_bool() {
        let query = query::load().filter(|f| f.filter("active", Cmp::Eq, true));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.active));
        assert_eq!(results.len(), 6);
    }

    fn filter_gt_score() {
        let query = query::load().filter(|f| f.filter("score", Cmp::Gt, 80.0));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();
        assert!(results.iter().all(|e| e.score > 80.0));
    }

    fn filter_le_level() {
        let query = query::load().filter(|f| f.filter("level", Cmp::Lte, 3));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.level <= 3));
        assert_eq!(results.len(), 7);
    }

    fn filter_ne_category() {
        let query = query::load().filter(|f| f.filter("category", Cmp::Ne, "B"));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.category != "B"));
        assert_eq!(results.len(), 6);
    }

    fn filter_and_group() {
        let query = query::load().filter(|f| {
            f.and_group(|b| {
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
        assert_eq!(results.len(), 5);
    }

    fn filter_or_group() {
        let query = query::load().filter(|f| {
            f.filter("category", Cmp::Eq, "A")
                .or("category", Cmp::Eq, "C")
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
        assert_eq!(results.len(), 6);
    }

    fn filter_nested_groups() {
        let query = query::load().filter(|f| {
            f.filter("active", Cmp::Eq, true).or_group(|b| {
                b.and_group(|b| b.filter("score", Cmp::Lt, 40.0))
                    .or("offset", Cmp::Lt, 0)
            })
        });

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert!(!results.is_empty());
        assert_eq!(results.len(), 7);
    }

    fn filter_startswith_name() {
        let query = query::load().filter(|f| f.filter("name", Cmp::StartsWith, "A"));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.name.starts_with('A')));
        assert_eq!(results.len(), 1);
    }

    fn filter_not_clause() {
        let expr = FilterExpr::Clause(FilterClause::new("category", Cmp::Eq, "C")).not();
        let query = query::load().filter(|f| f.expr(expr));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.category != "C"));
        assert_eq!(results.len(), 7);
    }

    fn filter_true_short_circuit() {
        let query = query::load().filter(|f| f.expr(FilterExpr::True));
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
            .execute::<Filterable>(query::load().filter(|f| f.expr(FilterExpr::False)))
            .unwrap()
            .entities();

        assert_eq!(results.len(), 0);
    }

    fn filter_empty_result() {
        let query = query::load().filter(|f| f.filter("category", Cmp::Eq, "Nonexistent"));

        let results = db!()
            .load()
            .execute::<Filterable>(query)
            .unwrap()
            .entities();

        assert_eq!(results.len(), 0);
    }

    fn filter_in_category() {
        let query =
            query::load().filter(|f| f.filter("category", Cmp::In, Value::list(&["A", "C"])));

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
        assert_eq!(results.len(), 6);
    }

    fn filter_allin_tags() {
        let query =
            query::load().filter(|f| f.filter("tags", Cmp::AllIn, Value::list(&["blue", "green"])));

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

        assert_eq!(results.len(), 3);
    }

    fn filter_anyin_tags() {
        let query =
            query::load().filter(|f| f.filter("tags", Cmp::AnyIn, Value::list(&["blue", "green"])));

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

        assert_eq!(results.len(), 5);
    }

    fn filter_eq_principal() {
        // Use dummy principal that matches the one used in fixtures
        let expected = Filterable::dummy_principal(1);

        let results = db!()
            .load()
            .filter::<Filterable>(|f| f.filter("pid", Cmp::Eq, expected))
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
        let query = query::load().filter(|f| f.filter("tags", Cmp::Contains, "green"));

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
        assert_eq!(results.len(), 4);
    }
}

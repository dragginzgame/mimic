use mimic::{
    core::traits::FilterView,
    db::query::{ContainsFilter, TextFilter, TextFilterAction, TextFilterOp},
    prelude::*,
    types::Principal,
};
use test_design::e2e::filter::{Filterable, FilterableEnum};

///
/// AutoFilterSuite
///

pub struct AutoFilterSuite;

impl AutoFilterSuite {
    pub fn test() {
        let tests: Vec<(&str, fn())> = vec![
            ("text_filter_equal_ci", Self::text_filter_equal_ci),
            (
                "text_filter_contains_case_sensitive_no_match",
                Self::text_filter_contains_case_sensitive_no_match,
            ),
            ("text_filter_contains_ci", Self::text_filter_contains_ci),
            (
                "contains_filter_any_in_blue",
                Self::contains_filter_any_in_blue,
            ),
            (
                "contains_filter_not_any_in_red_green",
                Self::contains_filter_not_any_in_red_green,
            ),
        ];

        Self::insert();

        for (name, test_fn) in tests {
            println!("Running test: {name}");
            test_fn();
        }
    }

    fn insert() {
        use FilterableEnum::{A, B, C};

        #[rustfmt::skip]
        #[allow(clippy::type_complexity)]
        let fixtures: [(&str, &str, bool, f64, u8, i32, &[&str], u64, FilterableEnum); 4] = [
            ("Alpha", "A", true, 91.5, 1, -5, &["red", "blue"], 1, A),
            ("Beta", "B", false, 64.0, 2, 0, &["blue"], 2, B),
            ("Delta", "B", true, 72.3, 3, 5, &["yellow"], 3, B),
            ("gamma", "C", true, 55.0, 4, -1, &["green"], 4, C),
        ];

        for (name, category, active, score, level, offset, tags, pid_index, abc) in fixtures {
            db!()
                .insert(Filterable {
                    name: name.into(),
                    category: category.into(),
                    active,
                    score: Decimal::from(score),
                    level,
                    opt_level: Some(level),
                    offset,
                    tags: tags.iter().map(ToString::to_string).collect(),
                    pid: Principal::dummy(pid_index as u8),
                    abc,
                    ..Default::default()
                })
                .unwrap();
        }
    }

    fn apply(filter: Filter<Filterable>) -> Vec<Filterable> {
        let expr = <Filterable as FilterView>::into_expr(filter);

        db!()
            .load::<Filterable>()
            .filter_expr(expr)
            .unwrap()
            .entities()
    }

    fn text_filter_equal_ci() {
        let filter = Filter::<Filterable> {
            name: Some(TextFilter {
                actions: vec![TextFilterAction {
                    op: TextFilterOp::Equal,
                    case_insensitive: true,
                    values: vec!["beta".into()],
                }],
                is_empty: None,
            }),
            ..Default::default()
        };

        let results = Self::apply(filter);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "Beta");
    }

    fn text_filter_contains_case_sensitive_no_match() {
        let filter = Filter::<Filterable> {
            name: Some(TextFilter {
                actions: vec![TextFilterAction {
                    op: TextFilterOp::Contains,
                    case_insensitive: false,
                    values: vec!["TA".into()],
                }],
                is_empty: None,
            }),
            ..Default::default()
        };

        let results = Self::apply(filter);

        assert!(
            results.is_empty(),
            "case-sensitive contains should not match lowercase data"
        );
    }

    fn text_filter_contains_ci() {
        let filter = Filter::<Filterable> {
            name: Some(TextFilter {
                actions: vec![TextFilterAction {
                    op: TextFilterOp::Contains,
                    case_insensitive: true,
                    values: vec!["TA".into()],
                }],
                is_empty: None,
            }),
            ..Default::default()
        };

        let mut names: Vec<_> = Self::apply(filter)
            .into_iter()
            .map(|entity| entity.name)
            .collect();
        names.sort();

        assert_eq!(names, vec!["Beta".to_string(), "Delta".to_string()]);
    }

    fn contains_filter_any_in_blue() {
        let filter = Filter::<Filterable> {
            tags: Some(ContainsFilter {
                any_in: Some(vec!["blue".into()]),
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut names: Vec<_> = Self::apply(filter)
            .into_iter()
            .map(|entity| entity.name)
            .collect();
        names.sort();

        assert_eq!(names, vec!["Alpha".to_string(), "Beta".to_string()]);
    }

    fn contains_filter_not_any_in_red_green() {
        let filter = Filter::<Filterable> {
            tags: Some(ContainsFilter {
                not_any_in: Some(vec!["red".into(), "green".into()]),
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut names: Vec<_> = Self::apply(filter)
            .into_iter()
            .map(|entity| entity.name)
            .collect();
        names.sort();

        assert_eq!(names, vec!["Beta".to_string(), "Delta".to_string()]);
    }
}

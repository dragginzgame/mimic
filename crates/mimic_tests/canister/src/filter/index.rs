use mimic::prelude::*;
use test_design::canister::filter::FilterableIndex;

///
/// IndexFilterTester
///

pub struct IndexFilterTester {}

impl IndexFilterTester {
    pub fn test() {
        let tests: Vec<(&str, fn())> = vec![
            // existing
            ("filter_eq_name_alpha", Self::filter_eq_name_alpha),
            ("filter_eq_name_zeta", Self::filter_eq_name_zeta),
            ("filter_ne_name_alpha", Self::filter_ne_name_alpha),
            ("filter_eq_name_missing", Self::filter_eq_name_missing),
            (
                "filter_eq_name_case_sensitive",
                Self::filter_eq_name_case_sensitive,
            ),
            // NEW — Option<Text> tests
            ("filter_is_null_name_opt", Self::filter_is_null_name_opt),
            (
                "filter_is_not_null_name_opt",
                Self::filter_is_not_null_name_opt,
            ),
            ("filter_eq_name_opt_beta", Self::filter_eq_name_opt_beta),
            ("filter_eq_name_opt_alpha", Self::filter_eq_name_opt_alpha),
            // NEW — Vec<Text> membership tests
            (
                "filter_contains_name_many_alpha",
                Self::filter_contains_name_many_alpha,
            ),
            (
                "filter_contains_name_many_blue",
                Self::filter_contains_name_many_blue,
            ),
            (
                "filter_contains_name_many_missing",
                Self::filter_contains_name_many_missing,
            ),
            (
                "filter_contains_both_blue_and_green",
                Self::filter_contains_both_blue_and_green,
            ),
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
        // (name, name_opt, name_many)
        let fixtures: [(&str, Option<&str>, &[&str]); 10] = [
            ("Alpha", None, &["alpha", "blue"]),
            ("Beta", Some("Beta"), &[]),
            ("Gamma", Some("Alpha"), &["red", "alpha"]),
            ("Delta", None, &[]),
            ("Epsilon", Some("Epsilon"), &["green", "blue"]),
            ("Zeta", None, &[]),
            ("Eta", Some("Beta"), &["blue"]),
            ("Theta", None, &["yellow"]),
            ("Iota", Some("Alpha"), &["alpha"]),
            ("Kappa", None, &["green", "blue"]),
        ];

        for (name, name_opt, name_many) in fixtures {
            EntityService::save_fixture(
                db!(),
                FilterableIndex {
                    name: name.into(),
                    name_opt: name_opt.map(ToString::to_string),
                    name_many: name_many.iter().map(ToString::to_string).collect(),
                    ..Default::default()
                },
            );
        }
    }

    ///
    /// NORMAL (Filterable)
    ///

    /// name == "Alpha" -> exactly one row
    fn filter_eq_name_alpha() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name", Cmp::Eq, "Alpha"))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.name == "Alpha"));
        assert_eq!(results.len(), 1);
    }

    /// name == "Zeta" -> exactly one row
    fn filter_eq_name_zeta() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name", Cmp::Eq, "Zeta"))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.name == "Zeta"));
        assert_eq!(results.len(), 1);
    }

    /// name != "Alpha" -> 9 rows (everything except Alpha)
    fn filter_ne_name_alpha() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name", Cmp::Ne, "Alpha"))
            .unwrap()
            .entities();

        assert!(results.iter().all(|e| e.name != "Alpha"));
        assert_eq!(results.len(), 9);
    }

    /// name == "Nope" -> 0 rows
    fn filter_eq_name_missing() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name", Cmp::Eq, "Nope"))
            .unwrap()
            .entities();

        assert!(results.is_empty());
    }

    /// Case-sensitivity: assuming raw byte collation (no normalization),
    /// "alpha" should not match "Alpha".
    fn filter_eq_name_case_sensitive() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name", Cmp::Eq, "alpha"))
            .unwrap()
            .entities();

        assert!(results.is_empty(), "expected case-sensitive match for name");
    }

    // ---------- Option<Text> tests ----------

    // Expect 5 rows: Alpha, Delta, Zeta, Theta, Kappa
    fn filter_is_null_name_opt() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name_opt", Cmp::IsNone, ()))
            .unwrap()
            .entities();

        assert_eq!(results.len(), 5);
        assert!(results.iter().all(|e| e.name_opt.is_none()));
    }

    // Complement of the above: 5 rows with Some(_)
    fn filter_is_not_null_name_opt() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name_opt", Cmp::IsSome, ()))
            .unwrap()
            .entities();

        assert_eq!(results.len(), 5);
        assert!(results.iter().all(|e| e.name_opt.is_some()));
    }

    // Two rows have Some("Beta"): Beta, Eta
    fn filter_eq_name_opt_beta() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name_opt", Cmp::Eq, "Beta"))
            .unwrap()
            .entities();

        assert_eq!(results.len(), 2);
        assert!(
            results
                .iter()
                .all(|e| e.name_opt.as_deref() == Some("Beta"))
        );
    }

    // Two rows have Some("Alpha"): Gamma, Iota
    fn filter_eq_name_opt_alpha() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name_opt", Cmp::Eq, "Alpha"))
            .unwrap()
            .entities();

        assert_eq!(results.len(), 2);
        assert!(
            results
                .iter()
                .all(|e| e.name_opt.as_deref() == Some("Alpha"))
        );
    }

    // ---------- Vec<Text> membership tests ----------

    // "alpha" appears in Alpha, Gamma, Iota
    fn filter_contains_name_many_alpha() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name_many", Cmp::Contains, "alpha"))
            .unwrap()
            .entities();

        assert_eq!(results.len(), 3);
        assert!(
            results
                .iter()
                .all(|e| e.name_many.iter().any(|t| t == "alpha"))
        );
    }

    // "blue" appears in Alpha, Epsilon, Eta, Kappa
    fn filter_contains_name_many_blue() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name_many", Cmp::Contains, "blue"))
            .unwrap()
            .entities();

        assert_eq!(results.len(), 4);
        assert!(
            results
                .iter()
                .all(|e| e.name_many.iter().any(|t| t == "blue"))
        );
    }

    // Missing token yields 0 results
    fn filter_contains_name_many_missing() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| f.filter("name_many", Cmp::Contains, "zzzz"))
            .unwrap()
            .entities();

        assert!(results.is_empty());
    }

    // Require both "blue" AND "green": expect Epsilon and Kappa
    fn filter_contains_both_blue_and_green() {
        let results = db!()
            .load()
            .filter::<FilterableIndex>(|f| {
                f.filter("name_many", Cmp::Contains, "blue").filter(
                    "name_many",
                    Cmp::Contains,
                    "green",
                )
            })
            .unwrap()
            .entities();

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|e| {
            e.name_many.iter().any(|t| t == "blue") && e.name_many.iter().any(|t| t == "green")
        }));
    }

    // (Optional) If your API has StartsWith for Text, keep this separate from list Contains:
    // fn filter_starts_name_alpha_prefix() { ... Cmp::StartsWith, "Al" ... }
}

use mimic::{prelude::*, types::Principal};
use mimic_test_design::e2e::filter::{Filterable, FilterableEnum, FilterableOpt};

pub fn insert_filterable_data() {
    use FilterableEnum::{A, B, C};

    #[rustfmt::skip]
    let fixtures = [
        ("Alpha", "A", true, 87.2, 1, -10, vec!["red", "blue"], 1, A),
        ("Beta", "B", false, 65.1, 2, 0, vec!["green"], 2, B),
        ("Gamma", "C", true, 92.5, 3, 10, vec!["red", "yellow"], 3, C),
        ("Delta", "B", false, 15.3, 2, 5, vec![], 4, B),
        ("Epsilon", "A", true, 75.0, 4, -5, vec!["green", "blue"], 5, A),
        ("Zeta", "C", false, 88.8, 5, 15, vec!["purple"], 6, C),
        ("Eta", "B", true, 30.5, 1, 8, vec!["red"], 7, B),
        ("Theta", "A", true, 99.9, 6, -20, vec!["blue", "green"], 8 ,A),
        ("Iota", "C", false, 42.0, 3, 0, vec!["yellow", "red"], 9, C),
        ("Kappa", "B", true, 50.0, 2, 3, vec!["green", "blue"], 10, B),
    ];

    for (name, category, active, score, level, offset, tags, pid_index, abc) in fixtures {
        db!()
            .insert(Filterable {
                name: name.into(),
                category: category.into(),
                active,
                score: Decimal::from(score),
                level,
                offset,
                tags: tags.iter().map(ToString::to_string).collect(),
                pid: Principal::dummy(pid_index),
                abc,
                ..Default::default()
            })
            .unwrap();
    }
}

pub fn insert_filterable_opt_data() {
    let fixtures = [
        (Some("Alice"), Some(1), Some(-10), Some(Principal::dummy(1))),
        (Some("Bob"), Some(2), Some(0), Some(Principal::dummy(2))),
        (None, Some(3), None, Some(Principal::dummy(3))),
        (Some("Charlie"), None, Some(5), None),
        (None, None, None, None),
    ];

    for (name, level, offset, pid) in fixtures {
        db!()
            .insert(FilterableOpt {
                name: name.map(str::to_string),
                level,
                offset,
                pid,
                ..Default::default()
            })
            .unwrap();
    }
}

pub fn seed_filter_data() {
    insert_filterable_data();
    insert_filterable_opt_data();
}

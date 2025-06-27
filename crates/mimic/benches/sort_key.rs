use bencher::*;
use mimic::{db::types::SortKey, ops::types::Value};

benchmark_group!(
    benchmarks,
    create_sort_key,
    compare_sort_key,
    compare_vec_value
);

// create_sort_key
fn create_sort_key(bench: &mut Bencher) {
    let key = "field_abc";
    let value = Some(Value::Text("value_xyz".to_string()));

    bench.iter(|| {
        let sort_key = SortKey::new(vec![(key, value.clone())]);
        std::hint::black_box(sort_key);
    });
}

// compare_sort_key
fn compare_sort_key(bench: &mut Bencher) {
    let a = SortKey::new(vec![(
        "field_abc",
        Some(Value::Text("value_xyz".to_string())),
    )]);
    let b = SortKey::new(vec![(
        "field_abc",
        Some(Value::Text("value_xyz".to_string())),
    )]);

    bench.iter(|| {
        let result = std::cmp::PartialEq::eq(&a, &b);
        std::hint::black_box(result);
    });
}

// compare_vec_value
fn compare_vec_value(bench: &mut Bencher) {
    let a: Vec<(String, Option<Value>)> = vec![(
        "field_abc".to_string(),
        Some(Value::Text("value_xyz".to_string())),
    )];
    let b = a.clone();

    bench.iter(|| {
        let result = a == b;
        std::hint::black_box(result);
    });
}

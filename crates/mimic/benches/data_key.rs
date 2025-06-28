use bencher::*;
use mimic::{core::value::IndexValue, db::store::DataKey};

benchmark_group!(
    benchmarks,
    create_data_key,
    compare_data_key,
    compare_vec_value
);

// create_data_key
fn create_data_key(bench: &mut Bencher) {
    let key = "field_abc";
    let value = Some(IndexValue::Text("value_xyz".to_string()));

    bench.iter(|| {
        let data_key = DataKey::new(vec![(key, value.clone())]);
        std::hint::black_box(data_key);
    });
}

// compare_data_key
fn compare_data_key(bench: &mut Bencher) {
    let a = DataKey::new(vec![(
        "field_abc",
        Some(IndexValue::Text("value_xyz".to_string())),
    )]);
    let b = DataKey::new(vec![(
        "field_abc",
        Some(IndexValue::Text("value_xyz".to_string())),
    )]);

    bench.iter(|| {
        let result = std::cmp::PartialEq::eq(&a, &b);
        std::hint::black_box(result);
    });
}

// compare_vec_value
fn compare_vec_value(bench: &mut Bencher) {
    let a: Vec<(String, Option<IndexValue>)> = vec![(
        "field_abc".to_string(),
        Some(IndexValue::Text("value_xyz".to_string())),
    )];
    let b = a.clone();

    bench.iter(|| {
        let result = a == b;
        std::hint::black_box(result);
    });
}

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
    let value = Some(IndexValue::Int(345_345));

    bench.iter(|| {
        let data_key = DataKey::new(vec![(key, value)]);
        std::hint::black_box(data_key);
    });
}

// compare_data_key
fn compare_data_key(bench: &mut Bencher) {
    let a = DataKey::new(vec![("field_abc", Some(IndexValue::Int(3)))]);
    let b = DataKey::new(vec![("field_abc", Some(IndexValue::Int(4)))]);

    bench.iter(|| {
        let result = std::cmp::PartialEq::eq(&a, &b);
        std::hint::black_box(result);
    });
}

// compare_vec_value
fn compare_vec_value(bench: &mut Bencher) {
    let a: Vec<(String, Option<IndexValue>)> =
        vec![("field_abc".to_string(), Some(IndexValue::Int(123_123)))];
    let b = a.clone();

    bench.iter(|| {
        let result = a == b;
        std::hint::black_box(result);
    });
}

use bencher::*;
use mimic::{core::Key, db::store::DataKey};

benchmark_group!(benchmarks, create_data_key, compare_data_key,);

// create_data_key
fn create_data_key(bench: &mut Bencher) {
    let path = "field_abc";
    let key = Key::Int(345_345);

    bench.iter(|| {
        let data_key = DataKey::new(path, key);
        std::hint::black_box(data_key);
    });
}

// compare_data_key
fn compare_data_key(bench: &mut Bencher) {
    let a = DataKey::new("field_abc", 3);
    let b = DataKey::new("field_abc", 4);

    bench.iter(|| {
        let result = std::cmp::PartialEq::eq(&a, &b);
        std::hint::black_box(result);
    });
}

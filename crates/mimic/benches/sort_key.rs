use bencher::*;
use mimic::db::types::SortKey;

benchmark_group!(
    benchmarks,
    create_sort_key,
    compare_sort_key,
    compare_vec_string
);

fn create_sort_key(bench: &mut Bencher) {
    let key = "field_abc";
    let value = Some("value_xyz");

    bench.iter(|| {
        let sort_key = SortKey::new(vec![(key, value)]);
        std::hint::black_box(sort_key);
    });
}

fn compare_sort_key(bench: &mut Bencher) {
    let a = SortKey::new(vec![("field_abc", Some("value_xyz"))]);
    let b = SortKey::new(vec![("field_abc", Some("value_xyz"))]);

    bench.iter(|| {
        let result = std::cmp::PartialEq::eq(&a, &b);
        std::hint::black_box(result);
    });
}

fn compare_vec_string(bench: &mut Bencher) {
    let a: Vec<(String, Option<String>)> =
        vec![("field_abc".to_string(), Some("value_xyz".to_string()))];
    let b = a.clone();

    bench.iter(|| {
        let result = a == b;
        std::hint::black_box(result);
    });
}

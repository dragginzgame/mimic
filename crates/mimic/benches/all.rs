extern crate bencher;

mod sort_key;

use bencher::benchmark_main;

benchmark_main!(sort_key::benchmarks);

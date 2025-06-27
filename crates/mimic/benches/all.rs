extern crate bencher;

mod data_key;

use bencher::benchmark_main;

benchmark_main!(data_key::benchmarks);

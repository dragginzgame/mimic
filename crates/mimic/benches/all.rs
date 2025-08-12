extern crate bencher;

mod blob;
mod data_key;

use bencher::benchmark_main;

benchmark_main!(data_key::benchmarks, blob::benchmarks);

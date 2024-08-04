pub use design; // this is a local design crate outside of mimic
pub use mimic_base; // should point to mimic/src/mimic_base

// main
fn main() {
    mimic::cli::run();
}

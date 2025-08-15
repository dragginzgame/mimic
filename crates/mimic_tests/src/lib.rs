//
// MIMIC TESTS
//

#[cfg(test)]
pub mod test {
    use icu::{Log, log};

    #[test]
    fn test_icu_crate() {
        log!(Log::Info, "icu v{}", icu::VERSION);
        log!(Log::Info, "mimic v{}", mimic::VERSION);
    }
}

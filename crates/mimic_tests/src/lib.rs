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
        assert!(!icu::VERSION.is_empty(), "icu VERSION should be non-empty");
        assert!(!mimic::VERSION.is_empty(), "mimic VERSION should be non-empty");
    }
}

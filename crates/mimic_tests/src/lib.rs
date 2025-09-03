//
// MIMIC TESTS
//

#[cfg(test)]
pub mod test {
    use test_design::fixture::rarity::Rarity;

    #[test]
    fn test_icu_crate() {
        let r = Rarity::default();

        let errs = mimic::core::validate(&r);
        if let Err(e) = errs {
            println!("{e}");
        }
    }
}

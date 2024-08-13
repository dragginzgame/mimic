use test_schema::sanitize::ClampRecord;

///
/// SanitizeTester
///

pub struct SanitizeTester {}

impl SanitizeTester {
    // test
    pub fn test() {
        Self::test_clamp();
    }

    //
    // TESTS
    //

    // test_clamp
    fn test_clamp() {
        // 0
        let mut r = ClampRecord::new(0);
        orm::sanitize(&mut r);
        assert!(r.value == 10.into());

        // 15
        let mut r = ClampRecord::new(15);
        orm::sanitize(&mut r);
        assert!(r.value == 15.into());

        // 25
        let mut r = ClampRecord::new(25);
        orm::sanitize(&mut r);
        assert!(r.value == 20.into());
    }
}

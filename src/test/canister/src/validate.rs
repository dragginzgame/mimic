use test_schema::validate::ValidateTest;

///
/// ValidateTester
///

pub struct ValidateTester {}

impl ValidateTester {
    // test
    pub fn test() {
        Self::test_record();
    }

    //
    // TESTS
    //

    // test_record
    fn test_record() {
        // ok
        let e = ValidateTest {
            multiple_ten: 30.into(),
            ..Default::default()
        };
        let res = mimic::orm::validate(&e);
        assert!(res.is_ok(), "{res:?}");
    }
}

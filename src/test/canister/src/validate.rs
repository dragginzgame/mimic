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
        };
        let res = mimic::orm::validate(&e);
        assert!(res.is_ok(), "{res:?}");

        // fail
        let e = ValidateTest {
            multiple_ten: 43.into(),
        };
        let res = mimic::orm::validate(&e);

        // check result is what we expected
        match res {
            Ok(()) => panic!("result is not an error"),
            Err(mimic::orm::Error::Validation { errors }) => {
                assert_eq!(errors.len(), 1, "one error expected");
            }
            Err(e) => panic!("unexpected error: {e}"),
        }
    }
}

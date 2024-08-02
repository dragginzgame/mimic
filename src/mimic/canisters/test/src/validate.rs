use mimic_base::types::test::validate::Validator;

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
        let e = Validator {
            guide: 5.into(),
            multiple_ten: 30.into(),
        };
        let res = orm::validate(&e);
        assert!(res.is_ok(), "{res:?}");

        // fail
        let e = Validator {
            guide: 0.into(),
            multiple_ten: 43.into(),
        };
        let res = orm::validate(&e);

        // check result is what we expected
        match res {
            Ok(()) => panic!("result is not an error"),
            Err(orm::Error::Validation { errors }) => {
                assert_eq!(
                    errors.len(),
                    2,
                    "both guide and multiple_ten fields expected to fail validation"
                );
            }
            Err(e) => panic!("unexpected error: {e}"),
        }
    }
}

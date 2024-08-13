use test_schema::validate::Validator;

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
            multiple_ten: 30.into(),
        };
        let res = orm::validate(&e);
        assert!(res.is_ok(), "{res:?}");

        // fail
        let e = Validator {
            multiple_ten: 43.into(),
        };
        let res = orm::validate(&e);

        // check result is what we expected
        match res {
            Ok(()) => panic!("result is not an error"),
            Err(orm::Error::Validation { errors }) => {
                assert_eq!(errors.len(), 1, "one error expected");
            }
            Err(e) => panic!("unexpected error: {e}"),
        }
    }
}

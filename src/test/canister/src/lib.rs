mod db;

#[cfg(test)]
mod default {
    use test_schema::default::Record;

    // test_record
    #[test]
    fn test_record() {
        let r = Record::default();

        assert_eq!(r.u8_value, 1);
        assert_eq!(r.u8_static_fn, 32);
    }
}

#[cfg(test)]
mod validate {
    use test_schema::validate::ValidateTest;

    // test_record
    #[test]
    fn test_record() {
        // ok
        let e = ValidateTest {
            multiple_ten: 30.into(),
            ..Default::default()
        };
        let res = mimic::orm::validate(&e);
        assert!(res.is_ok(), "{res:?}");

        // fail
        let e = ValidateTest {
            multiple_ten: 43.into(),
            ..Default::default()
        };
        let res = mimic::orm::validate(&e);

        // check result is what we expected
        match res {
            Ok(()) => panic!("result is not an error"),
            Err(mimic::orm::OrmError::Validation { errors }) => {
                assert_eq!(errors.len(), 1, "one error expected");
            }
            Err(e) => panic!("unexpected error: {e}"),
        }
    }
}

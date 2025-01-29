// test_record
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

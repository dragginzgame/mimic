use mimic::core::validate::validate;
use test_design::validate::*;

///
/// ValidateTester
///

pub struct ValidateTester {}

impl ValidateTester {
    // test
    pub fn test() {
        Self::test_collections();
        Self::test_decimal();
        Self::test_record();
    }

    //
    // TESTS
    //

    // test_collections
    // all of these use Lt(10) as the validator
    fn test_collections() {
        use test_design::collections::{ListValidated, MapValidated, SetValidated};

        //
        // PASS
        //

        // list
        let list = ListValidated::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let res = validate(&list);
        assert!(res.is_ok(), "{res:?}");

        // map
        let map = MapValidated::from(vec![(1, 2), (3, 4), (5, 6)]);
        let res = validate(&map);
        assert!(res.is_ok(), "{res:?}");

        // set
        let set = SetValidated::from(vec![1, 2, 2, 2, 2, 2, 3, 4, 8]);
        let res = validate(&set);
        assert!(res.is_ok(), "{res:?}");

        //
        // ERR
        //

        // list
        let list = ListValidated::from(vec![1, 25, 38]);
        let res = validate(&list);
        assert!(res.is_err(), "{res:?}");

        // map
        let map = MapValidated::from(vec![(1, 2), (113, 4), (5, 6)]);
        let res = validate(&map);
        assert!(res.is_err(), "{res:?}");

        // set
        let set = SetValidated::from(vec![1, 2, 2, 2, 52, 2, 3, 4, 8]);
        let res = validate(&set);
        assert!(res.is_err(), "{res:?}");
    }

    // test_decimal
    fn test_decimal() {
        // OK
        let d = DecimalMaxDp::from(1.2);
        let res = validate(&d);
        assert!(res.is_ok(), "{res:?}");

        // ERR
        let d = DecimalMaxDp::from(1.2453);
        let res = validate(&d);
        assert!(res.is_err(), "{res:?}");
    }

    // test_record
    fn test_record() {
        // ok
        let e = ValidateTest {
            multiple_ten: 30.into(),
            ltoe_ten: 5,
            gt_fifty: 80,
            ..Default::default()
        };
        let res = validate(&e);
        assert!(res.is_ok(), "{res:?}");
    }
}

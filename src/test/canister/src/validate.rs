use test_schema::validate::ValidateTest;

///
/// ValidateTester
///

pub struct ValidateTester {}

impl ValidateTester {
    // test
    pub fn test() {
        Self::test_collections();
        Self::test_record();
    }

    //
    // TESTS
    //

    // test_collections
    // all of these use Lt(10) as the validator
    fn test_collections() {
        use test_schema::collections::{ListValidated, MapValidated, SetValidated};

        //
        // PASS
        //

        // list
        let list = ListValidated::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let res = mimic::orm::validate(&list);
        assert!(res.is_ok(), "{res:?}");

        // map
        let map = MapValidated::from(vec![(1, 2), (3, 4), (5, 6)]);
        let res = mimic::orm::validate(&map);
        assert!(res.is_ok(), "{res:?}");

        // set
        let set = SetValidated::from(vec![1, 2, 2, 2, 2, 2, 3, 4, 8]);
        let res = mimic::orm::validate(&set);
        assert!(res.is_ok(), "{res:?}");

        //
        // ERR
        //

        // list
        let list = ListValidated::from(vec![1, 25, 38]);
        let res = mimic::orm::validate(&list);
        assert!(res.is_err(), "{res:?}");

        // map
        let map = MapValidated::from(vec![(1, 2), (113, 4), (5, 6)]);
        let res = mimic::orm::validate(&map);
        assert!(res.is_err(), "{res:?}");

        // set
        let set = SetValidated::from(vec![1, 2, 2, 2, 52, 2, 3, 4, 8]);
        let res = mimic::orm::validate(&set);
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
        let res = mimic::orm::validate(&e);
        assert!(res.is_ok(), "{res:?}");
    }
}

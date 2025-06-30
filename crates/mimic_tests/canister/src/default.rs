use test_design::canister::default::Record;

///
/// DefaultTester
///

pub struct DefaultTester {}

impl DefaultTester {
    // test
    pub fn test() {
        Self::test_record();
    }

    // test_record
    fn test_record() {
        let r = Record::default();

        assert_eq!(r.nat8_value, 1);
        assert_eq!(r.nat8_static_fn, 32);
    }
}

use test_schema::sanitize::ClampRecord;

//
// just a place to mess around with tests while developing
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_validates() {
        use test_schema::validate::{MultipleTenType, ValidateTest};

        let e = ValidateTest {
            multiple_ten: 5.into(),
            ..Default::default()
        };
        println!("{e:?}");

        let errs = mimic::orm::validate(&e);
        println!("{errs:?}");

        //

        let e = MultipleTenType::from(5);
        println!("{e:?}");

        let errs = mimic::orm::validate(&e);
        println!("{errs:?}");
    }

    //
    // TESTS
    //

    // test_clamp
    #[test]
    fn test_clamp() {
        // 0
        let mut r = ClampRecord::new(0);
        println!("{r:?}");
        mimic::orm::sanitize(&mut r);
        println!("{r:?}");
        assert!(r.value == 10.into());

        // 15
        let mut r = ClampRecord::new(15);
        mimic::orm::sanitize(&mut r);
        assert!(r.value == 15.into());

        // 25
        let mut r = ClampRecord::new(25);
        mimic::orm::sanitize(&mut r);
        assert!(r.value == 20.into());
    }
}

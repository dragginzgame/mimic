//
// just a place to mess around with tests while developing
//

#[cfg(test)]
mod tests {

    #[test]
    fn test_id_generates() {
        use test_schema::validate::ValidateTest;

        let e = ValidateTest {
            multiple_ten: 8.into(),
            ltoe_ten: 12,
            gt_fifty: 3,
            ..Default::default()
        };
        println!("{e:?}");

        let errs = mimic::validate(&e);
        println!("{errs:?}");
        if let Err(e) = errs {
            println!("{e}");
        }
    }

    #[test]
    fn test_default_validates() {
        use test_schema::validate::{MultipleTenType, ValidateTest};

        let e = ValidateTest {
            multiple_ten: 5.into(),
            ..Default::default()
        };
        println!("{e:?}");

        let errs = mimic::validate(&e);
        println!("{errs:?}");

        //

        let e = MultipleTenType::from(5);
        println!("{e:?}");

        let errs = mimic::validate(&e);
        println!("{errs:?}");
    }
}

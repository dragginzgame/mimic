//
// just a place to mess around with tests while developing
//

#[cfg(test)]
mod tests {

    #[test]
    fn test_default_validates() {
        use test_schema::validate::{MultipleTenType, ValidateTest};

        let e = ValidateTest {
            multiple_ten: 5.into(),
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
}

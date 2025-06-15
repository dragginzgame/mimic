use mimic::{def::validate, prelude::*};

//
// just a place to mess around with tests while developing
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_sort_keys() {
        use test_design::index::Index;

        let e = Index {
            x: 12,
            y: -200,
            ..Default::default()
        };
        println!("{e:?}");
        println!();

        println!("key : {:?}", e.key());
        println!("values : {:?}", e.values());
        println!("sort_key : {:?}", e.sort_key());
    }

    #[test]
    fn test_id_generates() {
        use test_design::validate::ValidateTest;

        let e = ValidateTest {
            multiple_ten: 8.into(),
            ltoe_ten: 12,
            gt_fifty: 3,
            ..Default::default()
        };
        println!("{e:?}");

        let errs = validate(&e);
        println!("{errs:?}");
        if let Err(e) = errs {
            println!("{e}");
        }
    }

    #[test]
    fn test_default_validates() {
        use test_design::validate::ValidateTest;

        let e = ValidateTest {
            multiple_ten: 5.into(),
            ..Default::default()
        };
        println!("{e:?}");

        let errs = validate(&e);
        println!("{errs:?}");
    }
}

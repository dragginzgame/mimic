use mimic::prelude::*;

//
// just a place to mess around with tests while developing
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ic_account() {
        use mimic::types::{Principal, Subaccount};
        use mimic_base::types::ic::Account;

        let e = Account {
            owner: Principal::anonymous(),
            subaccount: Some(Subaccount::from_u128s(0, 1000000)),
        };

        println!("{:?} {}", e, e.owner);
        if let Some(subaccount) = e.subaccount {
            println!("subaccount: {subaccount}");
        }
    }

    #[test]
    fn test_get_sort_keys() {
        use test_design::index::Index;

        let e = Index {
            x: 12,
            y: -200,
            ..Default::default()
        };
        println!("{e:?}");

        let sort_keys = e.key_values();
        println!("{sort_keys:?}");
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

        let errs = mimic::validate(&e);
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

        let errs = mimic::validate(&e);
        println!("{errs:?}");
    }
}

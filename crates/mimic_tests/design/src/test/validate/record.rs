use crate::prelude::*;

#[record(fields(
    field(
        ident = "duration_ms",
        value(item(
            prim = "Nat32",
            validator(path = "validator::num::Range", args(180000, 604800000))
        ))
    ),
    field(
        ident = "attempts",
        value(item(prim = "Nat32", validator(path = "validator::num::Range", args(1, 20))))
    ),
))]
pub struct VendorPolicy {}

#[cfg(test)]
mod tests {
    use super::*;
    use mimic::core::{ValidateError, validate};

    #[test]
    fn record_field_errors_include_field_names() {
        let policy = VendorPolicy {
            duration_ms: 100,
            attempts: 0,
        };

        let err = validate(&policy).expect_err("policy validation should fail");
        let ValidateError::ValidationFailed(tree) = err;
        let flat = tree.flatten_ref();

        assert!(
            flat.contains(&(
                "duration_ms".to_string(),
                "100 must be between 180000 and 604800000".to_string(),
            )),
            "missing duration_ms error: {flat:?}"
        );

        assert!(
            flat.contains(&(
                "attempts".to_string(),
                "0 must be between 1 and 20".to_string(),
            )),
            "missing attempts error: {flat:?}"
        );
    }
}

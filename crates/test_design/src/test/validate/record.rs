use crate::prelude::*;

///
/// Record
///

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
    field(
        ident = "bytes",
        value(item(prim = "Blob", validator(path = "validator::len::Max", args(500)))),
    )
))]
pub struct Record {}

///
/// TESTS
///

#[cfg(test)]
mod tests {
    use super::*;
    use icydb::core::{ValidateError, validate};

    #[test]
    fn base_record_validation_fields_fail_as_expected() {
        let r = Record {
            duration_ms: 100,             // invalid (too low)
            attempts: 0,                  // invalid (too low)
            bytes: vec![0u8; 600].into(), // invalid (too long)
        };

        let err = validate(&r).expect_err("validation should fail for invalid values");
        let ValidateError::ValidationFailed(tree) = err;
        let flat = tree.flatten_ref();

        // collect just the field names that failed
        let failed_fields: Vec<_> = flat.iter().map(|(field, _)| field.as_str()).collect();

        // verify all expected fields failed
        for field in ["duration_ms", "attempts", "bytes"] {
            assert!(
                failed_fields.contains(&field),
                "expected field `{field}` to fail validation, got {failed_fields:?}"
            );
        }
    }
}

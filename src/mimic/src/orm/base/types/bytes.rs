use crate::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// Utf8
///

#[newtype(
    primitive = "Blob",
    item(is = "types::Blob"),
    traits(remove(ValidateManual))
)]
pub struct Utf8 {}

#[allow(clippy::cast_possible_wrap)]
impl ValidateManual for Utf8 {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::default();

        // utf8
        errs.add_result(validator::bytes::Utf8 {}.validate(self));

        errs.result()
    }
}

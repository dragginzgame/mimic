use crate::design::{base, prelude::*};

///
/// Utf8
///

#[newtype(
    primitive = "Blob",
    item(prim = "Blob"),
    traits(remove(ValidateCustom))
)]
pub struct Utf8;

#[allow(clippy::cast_possible_wrap)]
impl ValidateCustom for Utf8 {
    fn validate_custom(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::default();

        // utf8
        errs.add_result(base::validator::bytes::Utf8.validate(self));

        errs.result()
    }
}

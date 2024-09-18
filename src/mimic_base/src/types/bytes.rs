use crate::{types, validator};
use mimic::orm::prelude::*;

///
/// Bytes
///

#[newtype(
    primitive = "Blob",
    value(item(is = "types::Blob")),
    traits(remove(ValidateManual))
)]
pub struct Bytes<const LEN: usize> {}

#[allow(clippy::cast_possible_wrap)]
impl<const LEN: usize> ValidateManual for Bytes<LEN> {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        let errs = ErrorVec::from_result(validator::len::Max::validate(
            &self.0.to_vec(),
            LEN as isize,
        ));

        errs.result()
    }
}

///
/// Utf8
///

#[newtype(
    primitive = "Blob",
    value(item(is = "types::Blob")),
    traits(remove(ValidateManual))
)]
pub struct Utf8<const LEN: usize> {}

#[allow(clippy::cast_possible_wrap)]
impl<const LEN: usize> ValidateManual for Utf8<LEN> {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::default();

        // utf8
        errs.add_result(validator::bytes::Utf8::validate(self));

        // len
        errs.add_result(validator::len::Max::validate(
            &self.0.to_vec(),
            LEN as isize,
        ));

        errs.result()
    }
}

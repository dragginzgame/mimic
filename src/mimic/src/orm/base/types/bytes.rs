use crate::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// Bytes
///

#[newtype(
    primitive = "Blob",
    item(is = "types::Blob"),
    traits(remove(ValidateManual))
)]
pub struct Bytes<const LEN: usize> {}

#[allow(clippy::cast_possible_wrap)]
impl<const LEN: usize> ValidateManual for Bytes<LEN> {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::default();

        // len
        errs.add_result(validator::string::len::Max::new(LEN).validate_blob(&self.0));

        errs.result()
    }
}

///
/// Utf8
///

#[newtype(
    primitive = "Blob",
    item(is = "types::Blob"),
    traits(remove(ValidateManual))
)]
pub struct Utf8<const LEN: usize> {}

#[allow(clippy::cast_possible_wrap)]
impl<const LEN: usize> ValidateManual for Utf8<LEN> {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::default();

        // utf8
        errs.add_result(validator::bytes::Utf8::default().validate_blob(&self.0));

        // len
        errs.add_result(validator::string::len::Max::new(LEN).validate_blob(&self.0));

        errs.result()
    }
}

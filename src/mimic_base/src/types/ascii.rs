use crate::{types, validator};
use mimic::orm::prelude::*;

///
/// Text
/// like text::Text but validates ASCII
///

#[newtype(
    primitive = "String",
    value(item(is = "types::String")),
    traits(add(Hash), remove(ValidateManual))
)]
pub struct Text<const LEN: usize> {}

#[allow(clippy::cast_possible_wrap)]
impl<const LEN: usize> ValidateManual for Text<LEN> {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // ascii check
        if !self.0.chars().all(|char| char.is_ascii() || char == '\0') {
            errs.add("invalid ascii character");
        }

        // length check
        errs.add_result(validator::len::Max::validate(&self.0, LEN as isize));

        errs.result()
    }
}

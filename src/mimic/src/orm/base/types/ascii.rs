use crate::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// Text
/// like text::Text but validates ASCII
///

#[newtype(
    primitive = "String",
    item(is = "types::String"),
    traits(add(Hash), remove(ValidateManual))
)]
pub struct Text {}

#[allow(clippy::cast_possible_wrap)]
impl ValidateManual for Text {
    fn validate_manual(&self) -> Result<(), ErrorVec> {
        let mut errs = ErrorVec::new();

        // ascii check
        if !self.0.chars().all(|char| char.is_ascii() || char == '\0') {
            errs.add("invalid ascii character");
        }

        errs.result()
    }
}

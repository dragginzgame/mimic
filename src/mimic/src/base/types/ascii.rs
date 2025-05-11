use crate::{base::types, prelude::*};

///
/// Text
/// like text::Text but validates ASCII
///

#[newtype(
    primitive = "Text",
    item(is = "types::Text"),
    traits(add(Hash), remove(ValidateCustom))
)]
pub struct Text {}

#[allow(clippy::cast_possible_wrap)]
impl ValidateCustom for Text {
    fn validate_custom(&self) -> Result<(), ErrorTree> {
        let mut errs = ErrorTree::new();

        // ascii check
        if !self.0.chars().all(|char| char.is_ascii() || char == '\0') {
            errs.add("invalid ascii character");
        }

        errs.result()
    }
}

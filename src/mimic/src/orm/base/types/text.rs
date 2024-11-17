use crate::orm::{
    base::{types, validator},
    prelude::*,
};

///
/// Text
///
/// a String where the length is restricted by the generic parameter
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
        let mut errs = ErrorVec::default();

        errs.add_result(validator::len::Max::new(LEN).validate_string(&self.0));

        errs.result()
    }
}

///
/// Function
///
/// 30 characters, snake_case
///

#[newtype(
    primitive = "String",
    value(item(is = "types::String")),
    validator(path = "validator::len::Range", args(3, 30)),
    validator(path = "validator::string::case::Snake"),
    traits(add(Hash))
)]
pub struct Function {}

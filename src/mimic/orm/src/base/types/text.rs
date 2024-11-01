use crate::prelude;

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
        ErrorVec::from_result(validator::len::Max::validate(&self.0, LEN as isize)).result()
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

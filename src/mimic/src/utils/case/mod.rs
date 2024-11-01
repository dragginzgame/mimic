use strum::Display;

mod constant;
mod snake;
mod title;

//
// utils/case
//
// all case operations come through here as we have to include
// multiple crates to get the desired behaviour
//

// Case
#[derive(Copy, Clone, Debug, Display)]
pub enum Case {
    Camel,
    Constant, // adheres to rust constant rules, more strict than UPPER_SNAKE
    Kebab,
    Lower,
    Snake,
    Title,
    Upper,
    UpperCamel, // or PascalCase
    UpperSnake, // or SCREAMING_SNAKE
    UpperKebab, // or TRAIN-CASE
}

//
// Casing
//

pub trait Casing<T: AsRef<str>> {
    fn to_case(&self, case: Case) -> String;
    fn is_case(&self, case: Case) -> bool;
}

impl<T: AsRef<str>> Casing<T> for T
where
    String: PartialEq<T>,
{
    // to_case
    // don't use convert_case:: Lower and Upper because they add spaces, and other
    // unexpected behaviour
    fn to_case(&self, case: Case) -> String {
        use convert_case as cc;

        match case {
            // rust
            Case::Lower => self.as_ref().to_lowercase(),
            Case::Upper => self.as_ref().to_uppercase(),

            // custom
            Case::Title => title::to_title_case(self.as_ref()),
            Case::Snake => snake::to_snake_case(self.as_ref()),
            Case::UpperSnake => snake::to_snake_case(self.as_ref()).to_uppercase(),
            Case::Constant => constant::to_constant_case(self.as_ref()).to_uppercase(),

            // convert_case
            Case::Camel => cc::Casing::to_case(self, cc::Case::Camel),
            Case::Kebab => cc::Casing::to_case(self, cc::Case::Kebab),
            Case::UpperCamel => cc::Casing::to_case(self, cc::Case::UpperCamel),
            Case::UpperKebab => cc::Casing::to_case(self, cc::Case::Kebab).to_uppercase(),
        }
    }

    // is_case
    fn is_case(&self, case: Case) -> bool {
        &self.to_case(case) == self
    }
}
